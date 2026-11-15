//! Negotiates quality-of-service settings during unicast establishment.
//!
//! TODO Reviewed ??-??-2026.

// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>

//!
//

use core::marker::PhantomData;
use std::str::FromStr;

use async_trait::async_trait;
use zenoh_buffers::reader::DidntRead;
use zenoh_buffers::reader::Reader;
use zenoh_buffers::writer::DidntWrite;
use zenoh_buffers::writer::Writer;
use zenoh_codec::RCodec;
use zenoh_codec::WCodec;
use zenoh_codec::Zenoh080;
use zenoh_core::zerror;
use zenoh_link::EndPoint;
use zenoh_protocol::core::Metadata;
use zenoh_protocol::core::Priority;
use zenoh_protocol::core::PriorityRange;
use zenoh_protocol::core::Reliability;
use zenoh_protocol::transport::init;
use zenoh_protocol::transport::open;
use zenoh_result::Error as ZError;
use zenoh_result::ZResult;

use crate::unicast::establishment::AcceptFsm;
use crate::unicast::establishment::OpenFsm;

// Extension Fsm

/// State machine for negotiating quality-of-service settings.
pub(crate) struct QoSFsm<'a> {

    /// First value in the pair.
    _a: PhantomData<&'a ()>,
}

impl QoSFsm<'_> {

    /// Creates the stateless state machine used to negotiate QoS extensions.
    pub(crate) const fn new() -> Self {
        Self { _a: PhantomData }
    }
}

/// Defines the outcomes available to the ext workflow, including no qo s.
#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    NoQoS,
    QoS {

        /// Optional reliability used by qo sfsm; `None` means the value is not configured or
        /// unavailable.
        reliability: Option<Reliability>,

        /// Optional priorities used by qo sfsm; `None` means the value is not configured or
        /// unavailable.
        priorities: Option<PriorityRange>,
    },
}

impl State {

    /// Creates no-QoS state or reads the endpoint's QoS settings for negotiation.
    ///
    /// # Arguments
///
    /// * `is_qos` - Whether QoS negotiation is enabled for the transport.
    /// * `endpoint` - Endpoint metadata containing optional reliability and priority settings.
    ///
    /// # Errors
///
    /// Returns an error if the configured reliability or priority metadata is invalid.
    fn new(is_qos: bool, endpoint: &EndPoint) -> ZResult<Self> {
        if !is_qos {
            Ok(State::NoQoS)
        } else {
            let metadata = endpoint.metadata();

            let reliability = metadata
                .get(Metadata::RELIABILITY)
                .map(Reliability::from_str)
                .transpose()?;

            let priorities = metadata
                .get(Metadata::PRIORITIES)
                .map(PriorityRange::from_str)
                .transpose()?;

            Ok(State::QoS {
                priorities,
                reliability,
            })
        }
    }

    /// Attempts to from u64 and reports failure to the caller.
    fn try_from_u64(value: u64) -> ZResult<Self> {
        match value {
            0b000_u64 => Ok(State::NoQoS),
            0b001_u64 => Ok(State::QoS {
                priorities: None,
                reliability: None,
            }),
            value if value & 0b110_u64 != 0 => {
                let tag = value & 0b111_u64;

                let priorities = if tag & 0b010_u64 != 0 {
                    let start = Priority::try_from(((value >> 3) & 0xff) as u8)?;
                    let end = Priority::try_from(((value >> (3 + 8)) & 0xff) as u8)?;

                    Some(PriorityRange::new(start..=end))
                } else {
                    None
                };

                let reliability = if tag & 0b100_u64 != 0 {
                    let bit = ((value >> (3 + 8 + 8)) & 0x1) as u8 == 1;

                    Some(Reliability::from(bit))
                } else {
                    None
                };

                Ok(State::QoS {
                    priorities,
                    reliability,
                })
            }
            _ => Err(zerror!("invalid QoS").into()),
        }
    }

    /// Encodes [`QoS`] as a [`u64`].
    /// This function is used for encoding both of [`StateAccept`] in
    /// [`establishment::cookie::Cookie::ext_qos`](crate::unicast::establishment::cookie::Cookie::ext_qos) and
    /// [`zenoh_protocol::transport::init::ext::QoS`].
    /// The three least significant bits are used to discrimnate five states:
    /// 1. QoS is disabled
    /// 2. QoS is enabled but no priority range and no reliability setting are available
    /// 3. QoS is enabled and priority range is available but reliability is unavailable
    /// 4. QoS is enabled and reliability is available but priority range is unavailable
    /// 5. QoS is enabled and both priority range and reliability are available
    fn to_u64(&self) -> u64 {
        match self {
            State::NoQoS => 0b000_u64,
            State::QoS {
                priorities,
                reliability,
            } => {
                if reliability.is_none() && priorities.is_none() {
                    return 0b001_u64;
                }

                let mut value = 0b000_u64;

                if let Some(priorities) = priorities {
                    value |= 0b010_u64;
                    value |= (*priorities.start() as u64) << 3;
                    value |= (*priorities.end() as u64) << (3 + 8);
                }

                if let Some(reliability) = reliability {
                    value |= 0b100_u64;
                    value |= (bool::from(*reliability) as u64) << (3 + 8 + 8);
                }

                value
            }
        }
    }

    /// Converts between the state representation and exts while preserving the invariants required
    /// by callers.
    fn to_exts(&self) -> (Option<init::ext::QoS>, Option<init::ext::QoSLink>) {
        match self {
            State::NoQoS => (None, None),
            State::QoS {
                reliability: None,
                priorities: None,
            } => (Some(init::ext::QoS::new()), None),
            State::QoS {
                reliability: Some(_),
                ..
            }
            | State::QoS {
                priorities: Some(_),
                ..
            } => (None, Some(init::ext::QoSLink::new(self.to_u64()))),
        }
    }

    fn try_from_exts(
        (qos, qos_link): (Option<init::ext::QoS>, Option<init::ext::QoSLink>),
    ) -> ZResult<Self> {
        match (qos, qos_link) {
            (Some(_), Some(_)) => Err(zerror!(
                "Extensions QoS and QoSOptimized cannot both be enabled at once"
            )
            .into()),
            (Some(_), None) => Ok(State::QoS {
                reliability: None,
                priorities: None,
            }),
            (None, Some(qos)) => State::try_from_u64(qos.value),
            (None, None) => Ok(State::NoQoS),
        }
    }

    /// Reports whether the state meets the qos condition so callers can choose the correct branch.
    fn is_qos(&self) -> bool {
        matches!(self, State::QoS { .. })
    }

    fn priorities(&self) -> Option<PriorityRange> {
        match self {
            State::NoQoS
            | State::QoS {
                priorities: None, ..
            } => None,
            State::QoS {
                priorities: Some(priorities),
                ..
            } => Some(priorities.clone()),
        }
    }

    fn reliability(&self) -> Option<Reliability> {
        match self {
            State::NoQoS
            | State::QoS {
                reliability: None, ..
            } => None,
            State::QoS {
                reliability: Some(reliability),
                ..
            } => Some(*reliability),
        }
    }

    #[cfg(test)]
    fn rand() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            State::NoQoS
        } else {
            let priorities = rng.gen_bool(0.5).then(PriorityRange::rand);
            let reliability = rng
                .gen_bool(0.5)
                .then(|| Reliability::from(rng.gen_bool(0.5)));

            State::QoS {
                priorities,
                reliability,
            }
        }
    }
}

/// Authentication state maintained by the initiator during the open handshake.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StateOpen(State);

impl From<State> for StateOpen {
    fn from(value: State) -> Self {
        StateOpen(value)
    }
}

impl StateOpen {

    /// Creates opening-side QoS state from the endpoint's reliability and priority settings.
    ///
    /// # Arguments
///
    /// * `is_qos` - Whether QoS negotiation is enabled for the transport.
    /// * `endpoint` - Endpoint metadata containing optional reliability and priority settings.
    ///
    /// # Errors
///
    /// Returns an error if the endpoint contains invalid reliability or priority metadata.
    pub(crate) fn new(is_qos: bool, endpoint: &EndPoint) -> ZResult<Self> {
        State::new(is_qos, endpoint).map(StateOpen)
    }

    /// Reports whether the state open meets the qos condition so callers can choose the correct
    /// branch.
    pub(crate) fn is_qos(&self) -> bool {
        self.0.is_qos()
    }

    pub(crate) fn priorities(&self) -> Option<PriorityRange> {
        self.0.priorities()
    }

    pub(crate) fn reliability(&self) -> Option<Reliability> {
        self.0.reliability()
    }
}

#[async_trait]
impl<'a> OpenFsm for &'a QoSFsm<'a> {
    type Error = ZError;
    type RecvInitAckIn = (
        &'a mut StateOpen,
        (Option<init::ext::QoS>, Option<init::ext::QoSLink>),
    );
    type RecvInitAckOut = ();
    type RecvOpenAckIn = (&'a mut StateOpen, Option<open::ext::QoS>);
    type RecvOpenAckOut = ();
    type SendInitSynIn = &'a StateOpen;
    type SendInitSynOut = (Option<init::ext::QoS>, Option<init::ext::QoSLink>);
    type SendOpenSynIn = &'a StateOpen;
    type SendOpenSynOut = Option<open::ext::QoS>;

    /// Adds the opening peer's QoS state extensions to its InitSyn message.
///
    /// # Arguments
///
    /// * `state` - Opening-side state containing the configured QoS settings.
    async fn send_init_syn(
        self,
        state: Self::SendInitSynIn,
    ) -> Result<Self::SendInitSynOut, Self::Error> {
        Ok(state.0.to_exts())
    }

    /// Validates and stores the QoS settings advertised in the peer's InitAck.
    ///
    /// # Arguments
///
    /// * `input` - Opening-side state and optional QoS extensions received in InitAck.
    ///
    /// # Errors
///
    /// Returns an error if the peer advertises invalid QoS extensions or incompatible settings.
    async fn recv_init_ack(
        self,
        input: Self::RecvInitAckIn,
    ) -> Result<Self::RecvInitAckOut, Self::Error> {
        let (state_self, other_ext) = input;

        let state_other = State::try_from_exts(other_ext)?;

        let (
            State::QoS {
                reliability: self_reliability,
                priorities: self_priorities,
            },
            State::QoS {
                reliability: other_reliability,
                priorities: other_priorities,
            },
        ) = (state_self.0.clone(), state_other)
        else {
            *state_self = State::NoQoS.into();
            return Ok(());
        };

        let priorities = match (self_priorities, other_priorities) {
            (None, priorities) | (priorities, None) => priorities,
            (Some(self_priorities), Some(other_priorities)) => {
                if other_priorities.includes(&self_priorities) {
                    Some(self_priorities)
                } else {
                    return Err(zerror!(
                        "The PriorityRange received in InitAck is not a superset of my PriorityRange"
                    )
                    .into());
                }
            }
        };

        let reliability = match (self_reliability, other_reliability) {
            (None, reliability) | (reliability, None) => reliability,
            (Some(self_reliability), Some(other_reliability)) => {
                if self_reliability == other_reliability {
                    Some(self_reliability)
                } else {
                    return Err(zerror!(
                        "The Reliability received in InitAck doesn't match my Reliability"
                    )
                    .into());
                }
            }
        };

        *state_self = State::QoS {
            reliability,
            priorities,
        }
        .into();

        Ok(())
    }

    /// Omits a QoS extension from OpenSyn because QoS is negotiated in the earlier handshake step.
///
    /// # Arguments
///
    /// * `_state` - Opening-side QoS state; no extension is sent at this handshake step.
    async fn send_open_syn(
        self,
        _state: Self::SendOpenSynIn,
    ) -> Result<Self::SendOpenSynOut, Self::Error> {
        Ok(None)
    }

    /// Completes the QoS OpenAck step without changing the negotiated state.
///
    /// # Arguments
///
    /// * `_state` - Opening-side state and optional OpenAck QoS extension; neither changes state.
    async fn recv_open_ack(
        self,
        _state: Self::RecvOpenAckIn,
    ) -> Result<Self::RecvOpenAckOut, Self::Error> {
        Ok(())
    }
}

/// Quality-of-service extension state exchanged during unicast establishment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct StateAccept(State);

impl From<State> for StateAccept {
    fn from(value: State) -> Self {
        StateAccept(value)
    }
}

impl StateAccept {

    /// Creates accept-side QoS state from the endpoint's reliability and priority settings.
    ///
    /// # Arguments
///
    /// * `is_qos` - Whether QoS negotiation is enabled for the transport.
    /// * `endpoint` - Endpoint metadata containing optional reliability and priority settings.
    ///
    /// # Errors
///
    /// Returns an error if the endpoint contains invalid reliability or priority metadata.
    pub(crate) fn new(is_qos: bool, endpoint: &EndPoint) -> ZResult<Self> {
        State::new(is_qos, endpoint).map(StateAccept::from)
    }

    /// Reports whether the state accept meets the qos condition so callers can choose the correct
    /// branch.
    pub(crate) fn is_qos(&self) -> bool {
        self.0.is_qos()
    }

    pub(crate) fn priorities(&self) -> Option<PriorityRange> {
        self.0.priorities()
    }

    pub(crate) fn reliability(&self) -> Option<Reliability> {
        self.0.reliability()
    }

    #[cfg(test)]
    pub(crate) fn rand() -> Self {
        State::rand().into()
    }
}

// Codec
impl<W> WCodec<&StateAccept, &mut W> for Zenoh080
where
    W: Writer,
{
    type Output = Result<(), DidntWrite>;

    fn write(self, writer: &mut W, x: &StateAccept) -> Self::Output {
        self.write(writer, &x.0.to_u64())
    }
}

impl<R> RCodec<StateAccept, &mut R> for Zenoh080
where
    R: Reader,
{
    type Error = DidntRead;

    /// Reads and validates a QoS state value from the input stream.
///
    /// # Arguments
///
    /// * `reader` - Source that supplies the bytes or records consumed by this operation.
    ///
    /// # Errors
///
    /// Returns an error if the value cannot be decoded or does not represent a valid QoS state.
    fn read(self, reader: &mut R) -> Result<StateAccept, Self::Error> {
        Ok(State::try_from_u64(self.read(reader)?)
            .map_err(|_| DidntRead)?
            .into())
    }
}

#[async_trait]
impl<'a> AcceptFsm for &'a QoSFsm<'a> {
    type Error = ZError;
    type RecvInitSynIn = (
        &'a mut StateAccept,
        (Option<init::ext::QoS>, Option<init::ext::QoSLink>),
    );
    type RecvInitSynOut = ();
    type RecvOpenSynIn = (&'a mut StateAccept, Option<open::ext::QoS>);
    type RecvOpenSynOut = ();
    type SendInitAckIn = &'a StateAccept;
    type SendInitAckOut = (Option<init::ext::QoS>, Option<init::ext::QoSLink>);
    type SendOpenAckIn = &'a StateAccept;
    type SendOpenAckOut = Option<open::ext::QoS>;

    /// Validates and stores the QoS settings advertised in the peer's InitSyn.
    ///
    /// # Arguments
///
    /// * `input` - Accept-side state and optional QoS extensions received in InitSyn.
    ///
    /// # Errors
///
    /// Returns an error if the peer's priority range or reliability conflicts with local settings.
    async fn recv_init_syn(
        self,
        input: Self::RecvInitSynIn,
    ) -> Result<Self::RecvInitSynOut, Self::Error> {
        let (state_self, other_ext) = input;

        let state_other = State::try_from_exts(other_ext)?;

        let (
            State::QoS {
                reliability: self_reliability,
                priorities: self_priorities,
            },
            State::QoS {
                reliability: other_reliability,
                priorities: other_priorities,
            },
        ) = (state_self.0.clone(), state_other)
        else {
            *state_self = State::NoQoS.into();
            return Ok(());
        };

        let priorities = match (self_priorities, other_priorities) {
            (None, priorities) | (priorities, None) => priorities,
            (Some(self_priorities), Some(other_priorities)) => {
                if self_priorities.includes(&other_priorities) {
                    Some(other_priorities)
                } else {
                    return Err(zerror!(
                        "The PriorityRange received in InitSyn is not a subset of my PriorityRange"
                    )
                    .into());
                }
            }
        };

        let reliability = match (self_reliability, other_reliability) {
            (None, reliability) | (reliability, None) => reliability,
            (Some(self_reliability), Some(other_reliability)) => {
                if self_reliability == other_reliability {
                    Some(other_reliability)
                } else {
                    return Err(zerror!(
                        "The Reliability received in InitSyn doesn't match my Reliability"
                    )
                    .into());
                }
            }
        };

        *state_self = State::QoS {
            reliability,
            priorities,
        }
        .into();

        Ok(())
    }

    /// Adds the acceptor's negotiated QoS state extensions to its InitAck message.
///
    /// # Arguments
///
    /// * `state` - Accept-side state containing the negotiated QoS settings.
    async fn send_init_ack(
        self,
        state: Self::SendInitAckIn,
    ) -> Result<Self::SendInitAckOut, Self::Error> {
        Ok(state.0.to_exts())
    }

    /// Accepts the OpenSyn without additional QoS state changes.
///
    /// # Arguments
///
    /// * `_state` - Accept-side state and optional OpenSyn extension, which are not changed here.
    async fn recv_open_syn(
        self,
        _state: Self::RecvOpenSynIn,
    ) -> Result<Self::RecvOpenSynOut, Self::Error> {
        Ok(())
    }

    /// Omits a QoS extension from OpenAck because no additional QoS data is sent at this step.
    ///
    /// # Arguments
///
    /// * `_state` - Accept-side negotiated state; no extension is emitted at this step.
    async fn send_open_ack(
        self,
        _state: Self::SendOpenAckIn,
    ) -> Result<Self::SendOpenAckOut, Self::Error> {
        Ok(None)
    }
}

/// Tests the `qos` behavior and its boundary cases.
/// behavior.
#[cfg(test)]
mod tests {
    use zenoh_protocol::core::PriorityRange;
    use zenoh_protocol::core::Reliability;
    use zenoh_result::ZResult;

    use super::QoSFsm;
    use super::State;
    use super::StateAccept;
    use super::StateOpen;
    use crate::unicast::establishment::AcceptFsm;
    use crate::unicast::establishment::OpenFsm;

    macro_rules! priority_range {
        ($start:literal, $end:literal) => {
            PriorityRange::new($start.try_into().unwrap()..=$end.try_into().unwrap())
        };
    }

    /// Exchanges and validates QoS handshake extensions for the opening and accepting peers.
    async fn test_negotiation(
        state_open: &mut StateOpen,
        state_accept: &mut StateAccept,
    ) -> ZResult<()> {
        let fsm = QoSFsm::new();

        let ext = fsm.send_init_syn(&*state_open).await?;
        fsm.recv_init_syn((state_accept, ext)).await?;

        let ext = fsm.send_init_ack(&*state_accept).await?;
        fsm.recv_init_ack((state_open, ext)).await?;

        Ok(())
    }

    /// Confirms both peers negotiate matching reliability and priority settings.
    async fn test_negotiation_ok(state_open: State, state_accept: State, state_expected: State) {
        let mut state_open = state_open.into();
        let mut state_accept = state_accept.into();

        match test_negotiation(&mut state_open, &mut state_accept).await {
            Err(err) => panic!("expected `Ok(())`, got: {err}"),
            Ok(()) => {
                assert_eq!(state_open.0, state_accept.0);
                assert_eq!(state_open.0, state_expected);
            }
        };
    }

    /// Confirms incompatible QoS settings are rejected.
    async fn test_negotiation_err(state_open: State, state_accept: State) {
        let mut state_open = state_open.into();
        let mut state_accept = state_accept.into();

        if let Ok(()) = test_negotiation(&mut state_open, &mut state_accept).await {
            panic!("expected `Err(_)`, got `Ok(())`")
        }
    }

    /// Negotiates successfully when neither peer sets a priority range.
    #[tokio::test]
    async fn test_priority_range_negotiation_scenario_1() {
        test_negotiation_ok(
            State::QoS {
                priorities: None,
                reliability: None,
            },
            State::QoS {
                priorities: None,
                reliability: None,
            },
            State::QoS {
                priorities: None,
                reliability: None,
            },
        )
        .await
    }

    /// Adopts the acceptor's priority range when the opener leaves it unset.
    #[tokio::test]
    async fn test_priority_range_negotiation_scenario_2() {
        test_negotiation_ok(
            State::QoS {
                priorities: None,
                reliability: None,
            },
            State::QoS {
                priorities: Some(priority_range!(1, 3)),
                reliability: None,
            },
            State::QoS {
                priorities: Some(priority_range!(1, 3)),
                reliability: None,
            },
        )
        .await
    }

    /// Adopts the opener's priority range when the acceptor leaves it unset.
    #[tokio::test]
    async fn test_priority_range_negotiation_scenario_3() {
        test_negotiation_ok(
            State::QoS {
                priorities: Some(priority_range!(1, 3)),
                reliability: None,
            },
            State::QoS {
                priorities: None,
                reliability: None,
            },
            State::QoS {
                priorities: Some(priority_range!(1, 3)),
                reliability: None,
            },
        )
        .await
    }

    /// Keeps matching priority ranges after negotiation.
    #[tokio::test]
    async fn test_priority_range_negotiation_scenario_4() {
        test_negotiation_ok(
            State::QoS {
                priorities: Some(priority_range!(1, 3)),
                reliability: None,
            },
            State::QoS {
                priorities: Some(priority_range!(1, 3)),
                reliability: None,
            },
            State::QoS {
                priorities: Some(priority_range!(1, 3)),
                reliability: None,
            },
        )
        .await
    }

    /// Uses the narrower range when both peers offer overlapping priority ranges.
    #[tokio::test]
    async fn test_priority_range_negotiation_scenario_5() {
        test_negotiation_ok(
            State::QoS {
                priorities: Some(priority_range!(1, 3)),
                reliability: None,
            },
            State::QoS {
                priorities: Some(priority_range!(0, 4)),
                reliability: None,
            },
            State::QoS {
                priorities: Some(priority_range!(1, 3)),
                reliability: None,
            },
        )
        .await
    }

    /// Rejects negotiation when the peers' priority ranges cannot be reconciled.
    #[tokio::test]
    async fn test_priority_range_negotiation_scenario_6() {
        test_negotiation_err(
            State::QoS {
                priorities: Some(priority_range!(1, 3)),
                reliability: None,
            },
            State::QoS {
                priorities: Some(priority_range!(2, 3)),
                reliability: None,
            },
        )
        .await
    }

    /// Adopts the acceptor's reliability when the opener leaves it unset.
    #[tokio::test]
    async fn test_reliability_negotiation_scenario_2() {
        test_negotiation_ok(
            State::QoS {
                reliability: None,
                priorities: None,
            },
            State::QoS {
                reliability: Some(Reliability::BestEffort),
                priorities: None,
            },
            State::QoS {
                reliability: Some(Reliability::BestEffort),
                priorities: None,
            },
        )
        .await
    }

    /// Rejects negotiation when peers request different reliability modes.
    #[tokio::test]
    async fn test_reliability_negotiation_scenario_3() {
        test_negotiation_err(
            State::QoS {
                reliability: Some(Reliability::Reliable),
                priorities: None,
            },
            State::QoS {
                reliability: Some(Reliability::BestEffort),
                priorities: None,
            },
        )
        .await
    }

    /// Keeps matching reliability modes after negotiation.
    #[tokio::test]
    async fn test_reliability_negotiation_scenario_4() {
        test_negotiation_ok(
            State::QoS {
                reliability: Some(Reliability::Reliable),
                priorities: None,
            },
            State::QoS {
                reliability: Some(Reliability::Reliable),
                priorities: None,
            },
            State::QoS {
                reliability: Some(Reliability::Reliable),
                priorities: None,
            },
        )
        .await
    }

    /// Rejects negotiation when peers request different reliability modes.
    #[tokio::test]
    async fn test_reliability_negotiation_scenario_5() {
        test_negotiation_err(
            State::QoS {
                reliability: Some(Reliability::BestEffort),
                priorities: None,
            },
            State::QoS {
                reliability: Some(Reliability::Reliable),
                priorities: None,
            },
        )
        .await
    }

    /// Negotiates the common reliability mode and overlapping priority range.
    #[tokio::test]
    async fn test_priority_range_and_reliability_negotiation_scenario_1() {
        test_negotiation_ok(
            State::QoS {
                reliability: Some(Reliability::BestEffort),
                priorities: Some(priority_range!(1, 3)),
            },
            State::QoS {
                reliability: Some(Reliability::BestEffort),
                priorities: Some(priority_range!(1, 4)),
            },
            State::QoS {
                reliability: Some(Reliability::BestEffort),
                priorities: Some(priority_range!(1, 3)),
            },
        )
        .await
    }
}
