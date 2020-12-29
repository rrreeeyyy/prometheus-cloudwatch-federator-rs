# prometheus-cloudwatch-federator-rs

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
