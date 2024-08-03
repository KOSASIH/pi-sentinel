use prometheus::{Prometheus, Gauge};
use grafana::{Grafana, Dashboard};

pub struct RealtimeAnalytics {
    prometheus: Prometheus,
    grafana: Grafana,
}

impl RealtimeAnalytics {
    pub fn new(prometheus_url: &str) -> Self {
        let prometheus = Prometheus::new(prometheus_url);
        let grafana = Grafana::new("http://localhost:3000");
        RealtimeAnalytics { prometheus, grafana }
    }

    pub fn report_block(&self, block: &Block) {
        // Report block metrics to Prometheus
        let gauge = self.prometheus.gauge("block_height");
        gauge.set(block.height as f64);

        // Create a new dashboard in Grafana
        let dashboard = self.grafana.create_dashboard("PiSentinel");
        dashboard.add_panel("Block Height", gauge);
    }
                            }
