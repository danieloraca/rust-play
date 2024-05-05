extern crate rusoto_core;
extern crate rusoto_rds;

use rusoto_core::Region;
use rusoto_rds::{DescribeDBInstancesMessage, Rds, RdsClient};

#[tokio::main]
async fn main() {
    let region = Region::EuWest1;
    let client = RdsClient::new(region);

    let request = DescribeDBInstancesMessage {
        ..Default::default()
    };

    match client.describe_db_instances(request).await {
        Ok(output) => {
            println!("DB Instances");
            for db_instance in output.db_instances.unwrap_or_default() {
                println!(
                    "DB Instance Identifier: {:?}",
                    db_instance.db_instance_identifier
                );
                println!("Master Username: {:?}", db_instance.master_username);
                println!("DB Name: {:?}", db_instance.db_name);
                println!(
                    "Read Replica Source DB Instance Identifier: {:?}",
                    db_instance.read_replica_source_db_instance_identifier
                );
                println!(
                    "Read Replica DB Instance Identifiers: {:?}",
                    db_instance.read_replica_db_instance_identifiers
                );
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
