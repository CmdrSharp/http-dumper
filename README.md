# HTTP Dumper

A simple web server that accepts most HTTP requests and dumps out their request headers and body for quick troubleshooting of API-driven applications.

## Installation

Installation is done via Helm.

```
helm repo add http-dumper https://cmdrsharp.github.io/http-dumper
helm repo update
helm install http-dumper http-dumper/httpdumper
```

See [values.yaml](charts/kubepulse/values.yaml) for a comprehensive list of values that can be set.
