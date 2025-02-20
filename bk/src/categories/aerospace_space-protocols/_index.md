# Space Protocols

[![cat-aerospace::space-protocols][cat-aerospace::space-protocols-badge]][cat-aerospace::space-protocols]{{hi:Space protocols}}

## Space protocols

{{#include space_protocols.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P3](https://github.com/john-cd/rust_howto/issues/202)

Common Space Protocols:

- CCSDS (Consultative Committee for Space Data Systems):  A suite of standards covering various aspects of space data systems, including:
  - Packet Telemetry:  For transmitting scientific data from spacecraft.
  - Telecommand:  For sending commands to spacecraft.
  - Space Link Protocols:  For managing communication links.
  - Navigation Data Messages (NADM): For precise orbit and time data exchange.
  - Tracking Data Message (TDM):  For tracking information of space objects.
  Rust Support:  Direct, comprehensive CCSDS crate support is limited.  You'll likely find fragments or need to build upon lower-level crates.

- SCOS-2000 (Spacecraft Control and Operation System-2000):  A European Space Agency (ESA) standard for spacecraft control.
  Rust Support: Very limited. FFI with existing ESA libraries (if available) or custom implementations are the likely paths.

- COP-1 (Command Operating Procedure-1): A CCSDS telecommand protocol.
  Rust Support: Likely needs a custom implementation based on the CCSDS specifications.

- PUS (Packet Utilization Standard): A CCSDS standard for on-board data handling.
  Rust Support: Likely needs a custom implementation based on the CCSDS specifications.

Link to Parsing:  Use crates like nom or binascii to parse binary data structures defined in the protocol specifications.
Link to Data Structures
Link to serde
Link to Encoding/Decoding
Link to

</div>
