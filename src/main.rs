mod args;
mod client;
mod config;
mod utils;

use std::collections::HashMap;
use std::path::Path;
use std::process::exit;
use std::{thread, time};

use args::Args;
use client::Client;
use dnspod_lib::prelude::*;

fn main() -> anyhow::Result<()> {
    let Args { cfg, dry, lrun } = Args::parse_args();
    let cfgfile = Path::new(&cfg);
    if !cfgfile.exists() {
        println!("Cfg File {} Not Exists.", cfg);
        exit(1)
    }
    let conf = config::read_config(cfg).unwrap();
    let client = Client::new(conf.secret_id.clone(), conf.secret_key.clone());
    let mut cache: HashMap<u64, String> = HashMap::new();

    loop {
        for item in &conf.subs {
            let ip_str = utils::get_ip(&conf.interface, &item.record_type).unwrap();
            let mut changed = false;
            // let key = format!("{}", item.record_id);
            match cache.get(&item.record_id) {
                Some(x) => {
                    if *x != ip_str {
                        // println!("x:{} ip_str:{}", *x, ip_str);
                        changed = true;
                    }
                }
                None => {
                    let res = client.execute(DescribeRecordList {
                        Domain: conf.domain.clone(),
                        Subdomain: Some(item.subdomain.clone()),
                        Keyword: None,
                    })?;
                    let record_list = res
                        .Response
                        .RecordList
                        .ok_or(anyhow::anyhow!("No record list returned!"))?;
                    for record in record_list {
                        if record.RecordId == item.record_id && record.Type == item.record_type {
                            if record.Value != ip_str {
                                changed = true;
                                println!(
                                    "ID: {} O: {} N: {}",
                                    item.record_id, record.Value, ip_str
                                );
                            }
                            cache.insert(item.record_id, ip_str.clone());
                        }
                    }
                }
            }

            if changed {
                if dry {
                    println!("DRY RUN set id: {} with ip: {}", item.record_id, ip_str);
                } else {
                    println!("ModifyDDNS set id: {} with ip: {}", item.record_id, ip_str);
                    client.execute(ModifyDynamicDNS {
                        Domain: conf.domain.clone(),
                        SubDomain: item.subdomain.clone(),
                        RecordId: item.record_id,
                        RecordLine: dnspod_lib::data_types::RecordLine::默认,
                        Value: ip_str,
                        Ttl: 60,
                    })?;
                }
            }
        }
        if lrun {
            thread::sleep(time::Duration::from_secs(30));
        } else {
            break;
        }
    }

    Ok(())
}
