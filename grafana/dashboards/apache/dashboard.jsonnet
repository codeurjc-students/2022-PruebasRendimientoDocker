local g = import 'github.com/grafana/grafonnet/gen/grafonnet-latest/main.libsonnet';

g.dashboard.new('Apache')
+ g.dashboard.withDescription("Dashboard for Apache Exporter")
+ g.dashboard.withPanels([
  g.panel.timeSeries.new('Total memory (bytes)')
  + g.panel.timeSeries.queryOptions.withTargets([
    g.query.prometheus.new(
      'Prometheus', 
      'machine_memory_bytes'
    ),
  ])
  + g.panel.timeSeries.gridPos.withW(24)
  + g.panel.timeSeries.gridPos.withH(8)
])