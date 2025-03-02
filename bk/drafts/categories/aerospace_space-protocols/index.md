# Space Protocols

[![cat-aerospace::space-protocols][cat-aerospace::space-protocols-badge]][cat-aerospace::space-protocols]{{hi:Space protocols}}

{{#include space_protocols.incl.md}}

## CCSDS (Consultative Committee for Space Data Systems)

It is a suite of standards covering various aspects of space data systems, including:

- Packet Telemetry: For transmitting scientific data from spacecraft.
- Telecommand: For sending commands to spacecraft.
- Space Link Protocols: For managing communication links.
- Navigation Data Messages (NADM): For precise orbit and time data exchange.
- Tracking Data Message (TDM): For tracking information of space objects.

Direct, comprehensive CCSDS crate support is limited. You'll likely find fragments or need to build upon lower-level crates.

## SCOS-2000 (Spacecraft Control and Operation System-2000)

SCOS-2000 is an European Space Agency (ESA) standard for spacecraft control.

Use FFI with existing ESA libraries. See [[development-tools_ffi | Development Tools: FFI]]

## COP-1 (Command Operating Procedure-1)

COP-1 is a CCSDS telecommand protocol. You will likely needs a custom implementation based on the CCSDS specifications.

## PUS (Packet Utilization Standard)

PUS is a CCSDS standard for on-board data handling. You will likely needs a custom implementation based on the CCSDS specifications.

## Parsing

Use crates like `nom` or `binascii` to parse binary data structures defined in the protocol specifications

See:

- [[parser-implementations | Parser Implementations]].
- [[parsing | Parsing]].

## Data Structures

See [[data-structures | Data Structures]].

## Encoding/Decoding

See [[encoding | Encoding]] and especially [[complex_encoding | Complex Encoding]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P3](https://github.com/john-cd/rust_howto/issues/202)

</div>
