# Testing with Artillery

- It's considered an anti-pattern to run artillery tests from developer machines, or directly from CI/CD nodes. A developer's local machine is great for great for developing test scripts themselves, but for running realistic and meaningful load tests always scale them out horizontally.

- To test UI test, it's possible to load test with Playwright and Artillery.

## References

- [Distributed load testing at scale with Artillery](https://www.artillery.io/docs/load-testing-at-scale)
  [Load testing with Playwright and Artillery](https://www.artillery.io/docs/reference/engines/playwright)
