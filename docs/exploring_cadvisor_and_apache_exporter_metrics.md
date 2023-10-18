# Exploring cAdvisor and Apache Exporter Metrics

## Overview

### cAdvisor CPU Metrics For Prometheus

<!-- 
- container_cpu_cfs_periods_total
- container_cpu_cfs_throttled_periods_total
- container_cpu_cfs_throttled_seconds_total
- container_cpu_load_average_10s
- container_cpu_schedstat_run_periods_total
- container_cpu_schedstat_runqueue_seconds_total
- container_cpu_schedstat_run_seconds_total
- container_cpu_system_seconds_total
- container_cpu_usage_seconds_total
- container_cpu_user_seconds_total
- container_spec_cpu_period
- container_spec_cpu_quota
- container_spec_cpu_shares
- machine_cpu_cache_capacity_bytes
- machine_cpu_cores
- machine_cpu_physical_cores
- machine_cpu_sockets
-->

### cAdvisor Load Metrics
<!-- 
- By default metric container_cpu_load_average_10s returns 0
- Activate with flag --enable_load_reader
- Inestable option
-->

#### Issues Encountered While Activating CPU Load Metrics

<!-- 
- Tested in WSL
- Tested in Manjaro (Arch based)
- tested in Ubuntu 22 (Debian based)
- Issue in cAdvisor talking about incompatibility with cgroup v2
-->

## References

[Kernel CPU Load](https://docs.kernel.org/admin-guide/cpu-load.html)
[Runtime Options for CPU](https://github.com/google/cadvisor/blob/master/docs/runtime_options.md#cpu)
[Linux Load Averages: Solving the Mystery](https://www.brendangregg.com/blog/2017-08-08/linux-load-averages.html)