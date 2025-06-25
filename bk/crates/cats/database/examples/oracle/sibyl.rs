#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// // Sibyl is an OCI-based interface between Rust applications and Oracle
// // databases. Sibyl supports both blocking (threads) and nonblocking (async)
// // API.

// // To use Sibyl, you need to download the appropriate Instant Client packages
// // for your Linux distribution and architecture (usually 64-bit) from the
// // official Oracle website:
// // <https://www.oracle.com/database/technologies/instant-client/downloads.html>
// // Install the alien package: This tool is used to convert RPM packages
// // (which Oracle provides) to Debian packages suitable for Ubuntu.
// // sudo apt update
// // sudo apt install alien libaio1
// // Extract the downloaded ZIP files: Choose a suitable directory (e.g.,
// /opt/oracle) and extract the downloaded ZIP files there.
// // sudo mkdir -p /opt/oracle
// // sudo unzip instantclient-basic-linux.x64-<version>.zip -d /opt/oracle
// // sudo unzip instantclient-sqlplus-linux.x64-<version>.zip -d
// // /opt/oracle
// // Convert RPM packages to DEB packages (if applicable):
// // If you downloaded RPM packages, convert them to DEB using the `alien`
// // command.
// // sudo alien -i oracle-instantclient-<version>-basic-<version>.rpm
// // sudo alien -i oracle-instantclient-<version>-sqlplus-<version>.rpm
// // Install the DEB packages (if applicable):
// // sudo dpkg -i oracle-instantclient*.deb
// // Update the library path:
// // sudo sh -c 'echo /opt/oracle/instantclient_<version> >
// // /etc/ld.so.conf.d/oracle-instantclient.conf' // sudo ldconfig
// // Set environment variables (optional):
// // export PATH=$PATH:/opt/oracle/instantclient_<version> (for `sqlplus`)
// // export LD_LIBRARY_PATH=/opt/oracle/instantclient_<version>:
// // $LD_LIBRARY_PATH
// // Try connecting to your Oracle database using sqlplus:
// // sqlplus <username>/<password>@<hostname>:<port>/<service_name>

// // Define the environment variables:
// // DBNAME = "db"
// // DBUSER = "user"
// // DBPASS = "passwd"

// // In your Cargo.toml, add:
// // sibyl = { version = "0.6.18", features = [ "blocking" ] }

// use sibyl as oracle;
// fn main()  -> anyhow::Result<()> {
//     // Create an Oracle environment
//     let oracle = oracle::env()?;
//     // Get database credentials from environment variables.
//     let dbname = std::env::var("DBNAME").expect("database name");
//     let dbuser = std::env::var("DBUSER").expect("user name");
//     let dbpass = std::env::var("DBPASS").expect("password");

// // Establishes a connection to the database using the provided
// // credentials
// let session = oracle.connect(&dbname, &dbuser, &dbpass)?;

//     // Prepare the SQL statement
//     let stmt = session.prepare("SELECT first_name, last_name
//                                 FROM hr.employees
//                                 WHERE department_id = :department_id")?;

//     // Executes the prepared statement with the department ID set to 10 and
//     // retrieves a Rows iterator
//     let rows = stmt.query(&10)?;

//     while let Some(row) = rows.next()? {
//         let first_name: Option<&str> = row.get(0)?;
//         let last_name: Option<&str> = row.get(1)?;

//         if let (Some(first), Some(last)) = (first_name, last_name) {
//             println!("{} {}", first, last);
//         }
//     }

//     Ok(())
// }

// #[test]
// fn require_external_svc() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// [finish; review https://lib.rs/crates/sibyl ; also test / review in depth install steps NOW](https://github.com/john-cd/rust_howto/issues/1022)
// // <https://docs.oracle.com/en/database/oracle/oracle-database/21/lacli/sdk-instant-client.html>
// // <https://docs.oracle.com/en/database/oracle/oracle-database/19/lacli/installing-ic-arm-packages.html>
// // <https://help.ubuntu.com/community/RPM/AlienHowto>
