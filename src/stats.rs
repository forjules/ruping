#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub attempts: u32,
    pub connects: u32,
    pub failures: u32,
    pub minimum: f64,
    pub maximum: f64,
    pub total: f64,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn average(&self) -> f64 {
        if self.attempts == 0 {
            0.0
        } else {
            self.total / (self.attempts as f64)
        }
    }

    pub fn update_max_min(&mut self, value: f64) {
        if self.connects == 1 {
            self.minimum = value;
            self.maximum = value;
            return;
        }

        if value < self.minimum {
            self.minimum = value;
        }
        if value > self.maximum {
            self.maximum = value;
        }
    }

    pub fn get_statistics_string(&self) -> String {
        let fail_percent = if self.attempts == 0 {
            0.0
        } else {
            ((self.failures as f64) / (self.attempts as f64)) * 100.0
        };

        format!(
            "\nConnection statistics:\n\
             \tAttempted = ${}$, Connected = ${}$, Failed = ${}$ (${:.2}%$)\n\
             Approximate connection times:\n\
             \tMinimum = ${:.2}ms$, Maximum = ${:.2}ms$, Average = ${:.2}ms$\n",
            self.attempts,
            self.connects,
            self.failures,
            fail_percent,
            self.minimum,
            self.maximum,
            self.average()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_averages_and_min_max() {
        let mut stats = Stats::new();
        stats.attempts = 1;
        stats.connects = 1;
        stats.total = 10.0;
        stats.update_max_min(10.0);

        stats.attempts = 2;
        stats.connects = 2;
        stats.total = 30.0;
        stats.update_max_min(20.0);

        assert_eq!(stats.minimum, 10.0);
        assert_eq!(stats.maximum, 20.0);
        assert_eq!(stats.average(), 15.0);

        let str_rep = stats.get_statistics_string();
        println!("str_rep:\n{}", str_rep);
        assert!(str_rep.contains("Attempted = $2$, Connected = $2$, Failed = $0$ ($0.00%$)"));
        assert!(str_rep.contains("Minimum = $10.00ms$, Maximum = $20.00ms$, Average = $15.00ms$"));
    }
}
