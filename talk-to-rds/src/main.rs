extern crate rusoto_core;
extern crate rusoto_rds;
use mysql::{prelude::Queryable, Pool};

use rusoto_core::Region;
use rusoto_rds::{DescribeDBInstancesMessage, Rds, RdsClient};

async fn list_rds_instances(
) -> Result<(), rusoto_core::RusotoError<rusoto_rds::DescribeDBInstancesError>> {
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
            Ok(())
        }
        Err(error) => {
            println!("Error: {:?}", error);
            Err(error)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), mysql::Error> {
    if let Err(_) = list_rds_instances().await {
        println!("Error listing RDS instances")
    }

    Ok(())
}
