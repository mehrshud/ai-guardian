from src.config.rs import Config;
from src.vulnerability_scanner.rs import VulnerabilityScanner;

use log::{debug, error, info};
use std::result;

pub struct AIGuardian {
    config: Config,
    vulnerability_scanner: VulnerabilityScanner,
}

impl AIGuardian {
    pub fn new(config: Config) -> AIGuardian {
        AIGuardian {
            config,
            vulnerability_scanner: VulnerabilityScanner::new(),
        }
    }

    pub fn scan_vulnerabilities(&self) -> result::Result<Vec<Vulnerability>, String> {
        info!("Scanning for vulnerabilities");
        match self.vulnerability_scanner.scan() {
            Ok(vulnerabilities) => {
                debug!("Vulnerabilities found: {:?}", vulnerabilities);
                Ok(vulnerabilities)
            }
            Err(err) => {
                error!("Error scanning for vulnerabilities: {}", err);
                Err(err.to_string())
            }
        }
    }

    pub fn get_performance_metrics(&self) -> result::Result<Vec<PerformanceMetric>, String> {
        info!("Retrieving performance metrics");
        // Simulate retrieving performance metrics
        let metrics = vec![
            PerformanceMetric {
                id: 1,
                metric: "response_time".to_string(),
                value: 100.0,
            },
            PerformanceMetric {
                id: 2,
                metric: "throughput".to_string(),
                value: 500.0,
            },
        ];
        debug!("Performance metrics: {:?}", metrics);
        Ok(metrics)
    }
}

#[derive(Debug)]
pub struct Vulnerability {
    pub id: i32,
    pub description: String,
    pub severity: String,
}

#[derive(Debug)]
pub struct PerformanceMetric {
    pub id: i32,
    pub metric: String,
    pub value: f64,
}