# Comparing Dashboards created using the UI vs Dashboards as Code 

## Introduction

There are multiple ways to create a dashboard in Grafana. We will explain some possible workflows to create a dashboard and save it so we can use version control to manage it.

You can find more information in the references section. 

### Using the UI

The easiest way to create a dashboard is through the user interface (UI). With a few clicks, you can design a dashboard and save it to a file. This method is user-friendly and doesn't require any technical expertise. By leveraging the UI, you can quickly create and save a customized dashboard that suits your needs.

Nevertheless, this method has some drawbacks. Copying and pasting the JSON is inefficient and error-prone. Also, the resulting JSON  is difficult to read and understand, making it hard to identify which changes were made just by examining the JSON.

### Dashboards as Code

Alternatively, you can create a dashboard using code. This way it is easier to manage and version control, being able to track changes and identify the differences between versions. Besides, it is easier to automate the creation of dashboards and to integrate them with other tools.

However, this method requires some technical expertise and it is not as user-friendly as the UI. In addition, it is not possible to create a dashboard using code without using a tool to generate the JSON and at present, there is no established tool to do this.

You can find more information about options to create dashboards as code in the references section.

## References

### Code tools to create Dashboards as Code 
- [A complete guide to managing Grafana as code: tools, tips, and tricks](https://grafana.com/blog/2022/12/06/a-complete-guide-to-managing-grafana-as-code-tools-tips-and-tricks/)
#### Using Grafonnet
- [Grafana as code](https://medium.com/@tarantool/grafana-as-code-b642cac9ae75)
- [Grafonnet repository](https://github.com/grafana/grafonnet-lib)
- [PromCon EU 2019: Managing Grafana Dashboards with grafonnet and git](https://www.youtube.com/watch?v=kV3Ua6guynI&ab_channel=PrometheusMonitoring)
