use prometheus_parse::{Labels, Scrape, Value};
use rusoto_cloudwatch::{CloudWatch, CloudWatchClient, Dimension, MetricDatum, PutMetricDataInput};
use rusoto_core::Region;
use std::env;

// Cloudwatch metric can be assigned up to 10 dimensions
// https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_Dimension.html
const DIMENSION_MAX: usize = 10;

// Each PutMetricData request is limited to no more than 20 different metrics
// https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_PutMetricData.html
const METRIC_DATA_CHUNK_SIZE: usize = 20;

const DEFAULT_CLOUDWATCH_NAMESPACE: &str = "Prometheus";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("PROMETHEUS_FEDERATE_URL")
        .expect("Environment variable PROMETHEUS_FEDERATE_URL must be given");
    let namespace =
        env::var("CLOUDWATCH_NAMESPACE").unwrap_or(DEFAULT_CLOUDWATCH_NAMESPACE.to_string());

    let response = reqwest::get(&url).await?;

    let body = response.text().await?;

    let lines: Vec<_> = body.lines().map(|s| Ok(s.to_owned())).collect();
    let scrape = Scrape::parse(lines.into_iter())?;

    let client = CloudWatchClient::new(Region::default());

    let metric_data = scrape_to_metric_data(&scrape);

    for chunk in metric_data.chunks(METRIC_DATA_CHUNK_SIZE) {
        let input = PutMetricDataInput {
            metric_data: chunk.to_vec(),
            namespace: namespace.to_string(),
        };

        client.put_metric_data(input).await?
    }

    Ok(())
}

fn scrape_to_metric_data(metrics: &Scrape) -> Vec<MetricDatum> {
    let metric_data: Vec<MetricDatum> = metrics
        .samples
        .iter()
        .map(|sample| MetricDatum {
            value: match sample.value {
                Value::Counter(v) | Value::Gauge(v) | Value::Untyped(v) => Some(v),
                _ => {
                    println!("Unsupported metric type");
                    Some(0.0)
                }
            },
            metric_name: sample.metric.to_string(),
            dimensions: Some(labels_to_dimensions(&sample.labels)),
            timestamp: Some(sample.timestamp.to_rfc3339()),
            ..Default::default()
        })
        .collect();

    metric_data
}

fn labels_to_dimensions(labels: &Labels) -> Vec<Dimension> {
    let mut dimensions: Vec<Dimension> = labels
        .iter()
        .filter(|(k, v)| !k.trim().is_empty() && !v.trim().is_empty())
        .map(|(k, v)| Dimension {
            name: k.to_string(),
            value: v.to_string(),
        })
        .collect();

    dimensions.sort_by(|x, y| x.name.cmp(&y.name));

    if dimensions.len() > DIMENSION_MAX {
        let rest: Vec<Dimension> = dimensions.drain(DIMENSION_MAX..).collect();
        println!(
            "Number of labels exceeds the dimensions max, truncate labels: #{:#?}",
            rest
        );
    }

    dimensions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_labels_to_dimentions() {
        let scrape = r#"
metrics{a="1",b=""} 1 1
metrics{a="1",b="2",c="3",d="4",e="5",f="6",g="7",h="8",i="9",j="10",k="11"} 1 1
"#;
        let br = BufReader::new(scrape.as_bytes());
        let mut s = Scrape::parse(br.lines()).unwrap().samples.into_iter();

        assert_eq!(labels_to_dimensions(&s.next().unwrap().labels).len(), 1);
        assert_eq!(labels_to_dimensions(&s.next().unwrap().labels).len(), 10);
    }
}
