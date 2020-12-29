# prometheus-cloudwatch-federator-rs

Federate your prometheus metrics into CloudWatch custom metrics using with [Federation](https://prometheus.io/docs/prometheus/latest/federation/) feature.

## Usage

```
AWS_ACCESS_KEY_ID="..." AWS_SECRET_ACCESS_KEY="..." CLOUDWATCH_NAMESPACE="Prometheus" PROMETHEUS_FEDERATE_URL="https://your-prometheus-server.example.com/federate?match%5B%5D=%7B__name__=~%22requests_total%22%7D" prometheus-cloudwatch-federator-rs
```

### Environment variables

- PROMETHEUS_FEDERATE_URL
    - must be given
    - must be urlencoded
- CLOUDWATCH_NAMESPACE
    - default: Prometheus

### IAM

- allow `cloudwatch:PutMetricData`

## Limitations

- [Histograms and summaries](https://prometheus.io/docs/practices/histograms/) are not supported
- More than 10 labels will be truncated due to [CloudWatch Cusutom Metrics dimension limitation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/APIReference/API_Dimension.html)
