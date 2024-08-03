# Architecture

Pi-Sentinel is designed to be a modular and scalable system for monitoring and alerting on cryptocurrency prices. The following components make up the architecture:

* **Price Fetcher**: Responsible for fetching real-time cryptocurrency prices from external APIs
* **Alert Engine**: Evaluates price data against customizable alert thresholds and triggers notifications
* **Notification Service**: Sends notifications to users via email, SMS, or other channels
* **Raspberry Pi Hardware**: Integrates with the Raspberry Pi's hardware components, such as the GPIO pins and display

The components interact with each other through a message queue, allowing for loose coupling and easy maintenance.

## Data Flow

The following diagram illustrates the data flow between components:
