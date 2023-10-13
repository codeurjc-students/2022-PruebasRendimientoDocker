# Metrics for Apache applications running in Docker

## Introduction

In this document, we will discuss some alternatives for obtaining metrics for Apache applications running in Docker in the Prometheus ecosystem. We will also look briefly at how metrics are monitored in Linux systems, and then explain the pros and cons of each approach.

Note that we will focus on the Prometheus ecosystem, so specific tools of other alternatives will not be discussed.

### Differences Between System and Application Monitoring

Before we dive into the specifics of monitoring Docker applications, let's clarify the fundamental difference between system monitoring and application monitoring, and understand why both are essential.

**System monitoring** provides insight into the overall health and performance of the host machine. It provides us with data on resource utilization, hardware health, and kernel-related metrics. This type of monitoring is critical to understanding system stability and operational efficiency.

**Application monitoring**, on the other hand, focuses on individual applications or services running within the system. It provides information about the resource consumption and performance of specific applications, enabling us to pinpoint applications that are consuming excessive resources and potentially causing disruptions.

It is important to recognize the unique roles of system and application monitoring because they serve different purposes. System monitoring ensures the overall stability of the infrastructure, while application monitoring allows us to identify and optimize resource-intensive applications.

**Docker containers** differ from traditional virtual machines in that they share the kernel and certain system metrics with the host system. As a result, standard system monitoring tools may not provide the expected insights when monitoring Docker containers. Effective monitoring of these containers requires specific tools tailored for containerized environments.

Understanding this fundamental difference is critical when monitoring Docker applications. The shared kernel and resource dynamics of Docker containers require specialized monitoring tools that can accurately capture container-specific metrics. These tools play a critical role in ensuring the health and performance of applications running inside Docker containers.

## System Monitoring

In the Prometheus ecosystem, **node_exporter** is a widely used system monitoring tool. Node_exporter collects a wide range of hardware and kernel related metrics from the host machine. Some of the key metrics it provides are

- CPU usage
- Memory usage
- Disk usage
- Network activity
- System load averages
- File system statistics

**node_exporter** provides a comprehensive view of host system performance and resource utilization. It's a valuable tool for ensuring that the underlying infrastructure supporting your Docker containers is healthy and performing optimally.

## Docker Monitoring

When it comes to monitoring Docker containers in the Prometheus ecosystem, there are several tools and approaches available to gather key metrics and insights.

### Docker Daemon Metrics

While it is possible to configure the Docker daemon as a Prometheus target, this primarily provides information about the status of the Docker daemon itself. This can be useful for understanding the overall health and performance of the Docker daemon, but it doesn't provide detailed, container-specific metrics. Therefore, for comprehensive container monitoring, we need to explore other solutions.

### cAdvisor

For in-depth container monitoring in Docker environments, **cAdvisor** stands out as the preferred choice, especially in Kubernetes environments. Designed specifically for container monitoring, **cAdvisor** provides valuable insight into the resource consumption and performance characteristics of running containers. Key metrics collected by cAdvisor include:

- CPU usage per container
- Memory usage per container
- Network statistics per container
- Filesystem statistics per container
- Container uptime and lifecycle events

One of the key benefits of **cAdvisor** is its ability to dive deep into individual containers, making it an indispensable tool for monitoring applications inside Docker containers. When integrated with Prometheus, it provides a comprehensive view of container performance, enabling efficient troubleshooting and optimization.

### Docker Stats

Docker provides a built-in command called `docker stats` that provides real-time monitoring of container resource usage. However, it's worth noting that this is not a native Prometheus-based solution. To export these metrics to Prometheus, you'll need to use a third-party exporter such as [docker-stats-exporter](https://github.com/wywywywy/docker_stats_exporter).

An advantage of using `docker stats` is its minimal resource footprint compared to cAdvisor. However, it provides fewer metrics and has limitations when dealing with large numbers of containers, such as in scenarios with hundreds of containers, because Docker creates a new process for each container. In such cases, the scalability of Docker statistics may be an issue.

## Apache Monitoring

Apache provides a module called **mod_status** that provides a simple HTML interface for monitoring the Apache server. It's important to note that this module is not enabled by default, but it can be enabled to access its metrics.

To export these Apache metrics to Prometheus, you can use the [Apache Exporter](https://github.com/Lusitaniae/apache_exporter). This tool allows you to collect and visualize Apache-specific metrics to gain insight into the performance and resource usage of your Apache applications running in Docker containers.

## Conclusion

In summary, monitoring Apache applications running in Docker containers within the Prometheus ecosystem requires a well-rounded approach. Both system and application monitoring are essential to ensure the overall health and performance of your infrastructure.

System monitoring, facilitated by tools like **node_exporter**, provides insight into the health of the host system, ensuring that the foundation supporting your Docker containers is stable.

For comprehensive Docker container monitoring, **cAdvisor** is ideal, allowing you to drill down into individual containers and collect key metrics. This is especially beneficial in Kubernetes environments.

In addition, monitoring Apache applications inside Docker containers is made possible by the mod_status module and the **Apache Exporter**. These tools help you understand the resource usage and performance of your Apache services.

By combining these approaches, you can create a robust monitoring strategy that enables you to proactively manage and optimize your Apache applications in Docker containers to ensure their efficient and reliable operation.

### Recommended Resources

If you're looking for more in-depth information and guidance on monitoring Apache applications in Docker containers, please refer to the following documents and articles:

1. **Setting Up the Apache Exporter**: Learn how to set up an environment to export Apache metrics to Prometheus with detailed instructions in this document: [Setting Up Apache Exporter](https://github.com/MarioRP-01/app-apache-php/blob/main/docs/export-apache-metrics-to-prometheus.md).

2. **Explore cAdvisor with Apache Exporter metrics**: Dive deeper into the available metrics and potential issues related to cAdvisor and Apache Exporter in this comprehensive documentation: [Exploring cAdvisor with Apache Exporter Metrics](./exploring_cadvisor_with_apache_exporter_metrics.md).

These resources provide valuable insights, step-by-step instructions, and in-depth knowledge to enhance your understanding of monitoring Apache applications in Docker containers and using Prometheus for effective performance management.

## References

[node_exporter Repository](https://github.com/prometheus/node_exporter)
[Collect Docker Daemon metrics with Prometheus](https://docs.docker.com/config/daemon/prometheus/)
[Available metrics for Prometheus](https://github.com/google/cadvisor/blob/master/docs/storage/prometheus.md)
[Linux alternatives to display metrics](https://phoenixnap.com/kb/check-cpu-usage-load-linux)
[Linux Load Averages: Solving the Mystery](https://www.brendangregg.com/blog/2017-08-08/linux-load-averages.html)