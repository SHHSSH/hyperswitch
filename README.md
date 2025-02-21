<p align="center">
  <img src="./docs/imgs/hyperswitch-logo-dark.svg#gh-dark-mode-only" alt="Hyperswitch-Logo" width="40%" />
  <img src="./docs/imgs/hyperswitch-logo-light.svg#gh-light-mode-only" alt="Hyperswitch-Logo" width="40%" />
</p>

Yes, you can use crypto with HyperSwitch.  HyperSwitch is designed to be a payment orchestration layer, meaning it connects to various payment processors and allows merchants to choose the best options for their needs, including cryptocurrency payments. Here's how it works, and why it's possible:

**Key Ways HyperSwitch Supports Crypto:**

1.  **Integration with Crypto Payment Processors:** HyperSwitch doesn't *directly* handle crypto transactions itself.  Instead, it integrates with specialized cryptocurrency payment processors.  Think of it like this: you wouldn't expect HyperSwitch to handle *every* type of credit card directly; it connects to Visa, Mastercard, etc., via payment processors.  The same principle applies to crypto.  Common examples of crypto payment processors that HyperSwitch *can* integrate with (or likely will in the future if they haven't already) include:
    *   **BitPay:** One of the largest and best-known crypto payment processors.
    *   **Coinbase Commerce:** Coinbase's offering for businesses to accept cryptocurrency.
    *   **Binance Pay:** Binance's payment solution.
    *   **BTCPay Server:** An open-source, self-hosted crypto payment processor (very important for decentralization).
    *   **Checkout.com Crypto:** Checkout.com has added crypto payment processing to its existing suite.
    *   **Stripe Crypto:**  In recent years Stripe has introduced support for crypto in certain jurisdictions.  HyperSwitch's ability to integrate with Stripe means it can indirectly facilitate crypto through this.
    *   **NOWPayments:** another crypto processor.
    *   **TripleA:** An example of a processor focused on the Asian market.
    * ...and many others.  The key is that HyperSwitch is designed to be *extensible*—to add new connectors as the payment landscape evolves.

2.  **Routing and Optimization:**  Just like with traditional payments, HyperSwitch can intelligently route crypto transactions.  This is a core feature of payment orchestration:
    *   **Cost Optimization:** Which crypto processor offers the lowest fees for a particular transaction (considering both the processor's fees and potential network fees)? HyperSwitch can help route to the cheapest option.
    *   **Success Rate Optimization:** If one crypto processor is experiencing downtime or has limitations, HyperSwitch can automatically switch to another. Some processors may have better success rates in certain geographic regions or with specific cryptocurrencies.
    *   **Compliance:** Different processors may have different compliance requirements (KYC/AML). A merchant might use HyperSwitch to choose a processor based on the level of compliance needed for a particular customer or transaction.

3.  **Unified Reporting and Reconciliation:** Even if a merchant uses multiple crypto payment processors (and traditional payment methods), HyperSwitch provides a single dashboard and reporting system. This simplifies accounting and reconciliation significantly.

4. **Token Support:** The specific cryptocurrencies supported depend on the *underlying payment processors* HyperSwitch is connected to.  If BitPay supports Bitcoin, Litecoin, and Ethereum, and HyperSwitch is connected to BitPay, then the merchant can accept those currencies.

**How a Transaction Might Work (Simplified Example):**

1.  A customer chooses to pay with Bitcoin on a merchant's website that uses HyperSwitch.
2.  HyperSwitch identifies the customer's location (e.g., for compliance reasons) and the desired cryptocurrency (Bitcoin).
3.  Based on pre-configured rules (set up by the merchant), HyperSwitch routes the transaction to, say, Coinbase Commerce.
4.  Coinbase Commerce generates a Bitcoin payment address and QR code.
5.  The customer sends Bitcoin to that address.
6.  Coinbase Commerce confirms the payment (handling the blockchain confirmation).
7.  Coinbase Commerce notifies HyperSwitch that the payment is successful (and potentially converts the crypto to fiat currency if the merchant wants that).
8.  HyperSwitch updates the merchant's system and provides reporting data.

**Important Considerations:**

*   **Specific Integrations:** While HyperSwitch *can* support crypto, it's crucial to check which specific crypto processors it *currently* has connectors for. This information is usually available in their documentation.
*   **Merchant Configuration:** The merchant needs to configure their HyperSwitch account to enable crypto payments and set up the rules for routing.
*   **Volatility:** Merchants need to be aware of the volatility of cryptocurrencies if they choose to hold the crypto rather than immediately converting it to fiat currency. HyperSwitch itself doesn't mitigate this risk; the choice of processor and the merchant's risk management strategy do.
*   **Regulatory Landscape:** Crypto regulations vary widely by country.  HyperSwitch can help with compliance *by connecting to compliant processors*, but the merchant ultimately remains responsible for ensuring they are operating legally.
* **Open Source:** HyperSwitch itself is open source. This is relevant because that allows you to inspect its codebase and contribute to it, including adding integrations to a crypto processor of your liking.

In short, HyperSwitch is designed to handle a wide range of payment methods, and cryptocurrency is absolutely within its capabilities through its integration with specialized payment processors. It offers the advantages of flexibility, optimization, and unified management, making crypto payments easier for merchants to implement and manage. Always refer to the latest HyperSwitch documentation for up-to-date specifics.

<h1 align="center">Open-Source Payments Orchestration</h1>

<div align="center" >
Single API to access the payments ecosystem and its features
</div>

<p align="center">
  <a href="https://github.com/juspay/hyperswitch/actions?query=workflow%3ACI+branch%3Amain">
    <img src="https://github.com/juspay/hyperswitch/workflows/CI-push/badge.svg" />
  </a>
  <a href="https://github.com/juspay/hyperswitch/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/juspay/hyperswitch" />
  </a>
  <a href="https://github.com/juspay/hyperswitch/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/Made_in-Rust-orange" />
  </a>
  <!-- Uncomment when we reach >50% coverage -->
  <!-- <a href="https://codecov.io/github/juspay/hyperswitch" >
    <img src="https://codecov.io/github/juspay/hyperswitch/graph/badge.svg"/>
  </a> -->
</p>
<p align="center">
  <a href="https://www.linkedin.com/company/hyperswitch/">
    <img src="https://img.shields.io/badge/follow-hyperswitch-blue?logo=linkedin&labelColor=grey"/>
  </a>
  <a href="https://x.com/hyperswitchio">
    <img src="https://img.shields.io/badge/follow-%40hyperswitchio-white?logo=x&labelColor=grey"/>
  </a>
  <a href="https://join.slack.com/t/hyperswitch-io/shared_invite/zt-2jqxmpsbm-WXUENx022HjNEy~Ark7Orw">
    <img src="https://img.shields.io/badge/chat-on_slack-blue?logo=slack&labelColor=grey&color=%233f0e40"/>
  </a>
</p>

<hr>

## Table of Contents

1. [Introduction](#introduction)
2. [Architectural Overview](#architectural-overview) 
3. [Try Hyperswitch](#try-hyperswitch)  
4. [Support, Feature requests & Bugs](#support-feature-requests)  
5. [Our Vision](#our-vision)  
6. [Versioning](#versioning)  
7. [Copyright and License](#copyright-and-license)

<a href="#introduction">
  <h2 id="introduction">Introduction</h2>
</a>
Juspay, founded in 2012, is a global leader in payment orchestration and checkout solutions, trusted by 400+ leading enterprises and brands worldwide. Hyperswitch is Juspay's new generation of composable, commercial open-source payments platform for merchant and brands. It is an enterprise-grade, transparent and modular payments platform designed to provide digital businesses access to the best payments infrastructure.

Here are the key components of Hyperswitch that deliver the whole solution:

* [Hyperswitch Backend](https://github.com/juspay/hyperswitch): Hyperswitch backend enables seamless payment processing with comprehensive support for various payment flows - authorization, authentication, void and capture workflows along with robust management of post-payment processes like refunds and chargeback handling. Additionally, Hyperswitch supports non-payment use cases by enabling connections with external FRM or authentication providers as part of the payment flow. The backend optimizes payment routing with customizable workflows, including success rate-based routing, rule-based routing, volume distribution, fallback handling, and intelligent retry mechanisms for failed payments based on specific error codes.

* [SDK (Frontend)](https://github.com/juspay/hyperswitch-web): The SDK, available for web, [Android, and iOS](https://github.com/juspay/hyperswitch-client-core), unifies the payment experience across various methods such as cards, wallets, BNPL, bank transfers, and more, while supporting the diverse payment flows of underlying PSPs. When paired with the locker, it surfaces the user's saved payment methods.    

* [Control Center](https://github.com/juspay/hyperswitch-control-center): The Control Center enables users to manage the entire payments stack without any coding. It allows the creation of workflows for routing, payment retries, and defining conditions to invoke 3DS, fraud risk management (FRM), and surcharge modules. The Control Center provides access to transaction, refund, and chargeback operations across all integrated PSPs, transaction-level logs for initial debugging, and detailed analytics and insights into payment performance.

Read more at [Hyperswitch docs](https://docs.hyperswitch.io/).

<a href="#architectural-overview">
  <h2 id="architectural-overview">Architectural Overview</h2>
</a>
<img src="./docs/imgs/features.png" />
<img src="./docs/imgs/non-functional-features.png" />

<img src="./docs/imgs/hyperswitch-architecture-v1.png" />

<a href="#try-hyperswitch">
  <h2 id="try-hyperswitch">Try Hyperswitch</h2>
</a>

### 1. Local Setup

You can run Hyperswitch on your system using Docker compose after cloning this repository. 

```shell
git clone --depth 1 --branch latest https://github.com/juspay/hyperswitch
cd hyperswitch
docker compose up -d
```

Check out the [local setup guide][local-setup-guide] for a more details on setting up the entire stack or component wise. This takes 15-mins and gives the following output 
```shell
[+] Running 2/2
✔ hyperswitch-control-center Pulled 2.9s
✔ hyperswitch-server Pulled 3.0s
[+] Running 6/0

✔ Container hyperswitch-pg-1 Created 0.0s
✔ Container hyperswitch-redis-standalone-1 Created 0.0s
✔ Container hyperswitch-migration_runner-1 Created 0.0s
✔ Container hyperswitch-hyperswitch-server-1 Created 0.0s
✔ Container hyperswitch-hyperswitch-web-1 Created 0.0s
✔ Container hyperswitch-hyperswitch-control-center-1 Created 0.0s

Attaching to hyperswitch-control-center-1, hyperswitch-server-1, hyperswitch-web-1, migration_runner-1, pg-1, redis-standalone-1
```

### 2. Deployment on cloud

The fastest and easiest way to try Hyperswitch on AWS is via our CDK scripts

1. Click on the following button for a quick standalone deployment on AWS, suitable for prototyping.
   No code or setup is required in your system and the deployment is covered within the AWS free-tier setup.

   <a href="https://console.aws.amazon.com/cloudformation/home?region=us-east-1#/stacks/new?stackName=HyperswitchBootstarp&templateURL=https://hyperswitch-synth.s3.eu-central-1.amazonaws.com/hs-starter-config.yaml"><img src="https://github.com/juspay/hyperswitch/blob/main/docs/imgs/aws_button.png?raw=true" height="35"></a>

2. Sign-in to your AWS console.

3. Follow the instructions provided on the console to successfully deploy Hyperswitch. This takes 30-45mins and gives the following output 

| Service| Host|
|----------------------------------------------|----------------------------------------------|
| App server running on                        | `http://hyperswitch-<host-id.region>.elb.amazonaws.com` |
| HyperloaderJS Hosted at                      | `http://<cloudfront.host-id>/0.103.1/v0/HyperLoader.js` |
| Control center server running on             | `http://hyperswitch-control-center-<host-id.region>.elb.amazonaws.com`, Login with Email: `test@gmail.com` |
| Hyperswitch Demo Store running on            | `http://hyperswitch-sdk-demo-<host-id.region>.elb.amazonaws.com` |
| Logs server running on                       | `http://hyperswitch-logs-<host-id.region>.elb.amazonaws.com`, Login with username: `admin`, password: `admin` |

We support deployment on GCP and Azure via Helm charts which takes 30-45mins. You can read more at [Hyperswitch docs](https://docs.hyperswitch.io/hyperswitch-open-source/deploy-on-kubernetes-using-helm). 

### 3. Hosted Sandbox

You can experience the product by signing up for our [hosted sandbox](https://app.hyperswitch.io/). The signup process accepts any email ID and provides access to the entire Control Center. You can set up connectors, define workflows for routing and retries, and even try payments from the dashboard.

[docs-link-for-enterprise]: https://docs.hyperswitch.io/hyperswitch-cloud/quickstart
[docs-link-for-developers]: https://docs.hyperswitch.io/hyperswitch-open-source/overview
[contributing-guidelines]: docs/CONTRIBUTING.md
[dashboard-link]: https://app.hyperswitch.io/
[website-link]: https://hyperswitch.io/
[learning-resources]: https://docs.hyperswitch.io/learn-more/payment-flows
[local-setup-guide]: /docs/try_local_system.md
[docker-compose-scheduler-monitoring]: /docs/try_local_system.md#running-additional-services


<a href="support-feature-requests">
  <h2 id="support-feature-requests">Support, Feature requests & Bugs</h2>
</a>

For any support, join the conversation in [Slack](https://join.slack.com/t/hyperswitch-io/shared_invite/zt-2jqxmpsbm-WXUENx022HjNEy~Ark7Orw)

For new product features, enhancements, roadmap discussions, or to share queries and ideas, visit our [GitHub Discussions](https://github.com/juspay/hyperswitch/discussions)

For reporting a bug, please read the issue guidelines and search for [existing and closed issues]. If your problem or idea is not addressed yet, please [open a new issue].

[existing and closed issues]: https://github.com/juspay/hyperswitch/issues
[open a new issue]: https://github.com/juspay/hyperswitch/issues/new/choose

<a href="our-vision">
  <h2 id="our-vision">Our Vision</h2>
</a>

> Linux for Payments

Payments are evolving rapidly worldwide, with hundreds of processors, fraud detection systems, authentication modules, and new payment methods and flows emerging. Businesses building or managing their own payment stacks often face similar challenges, struggle with comparable issues, and find it hard to innovate at the desired pace.

Hyperswitch serves as a well-architected designed reference platform, built on best-in-class design principles, empowering businesses to own and customize their payment stack. It provides a reusable core payments stack that can be tailored to specific requirements while relying on the Hyperswitch team for enhancements, support, and continuous innovation.

### Our Values

1. Embrace Payments Diversity: It will drive innovation in the ecosystem in
   multiple ways.
2. Make it Open Source: Increases trust; Improves the quality and reusability of
   software.
3. Be community driven: It enables participatory design and development.
4. Build it like Systems Software: This sets a high bar for Reliability,
   Security and Performance SLAs.
5. Maximise Value Creation: For developers, customers & partners.

This project is being created and maintained by [Juspay](https://juspay.io)

<a href="#versioning">
  <h2 id="versioning">Versioning</h2>
</a>

Check the [CHANGELOG.md](./CHANGELOG.md) file for details.

<a href="#copyright-and-license">
  <h2 id="copyright-and-license">Copyright and License</h2>
</a>

This product is licensed under the [Apache 2.0 License](LICENSE).


<a href="team-behind-hyperswitch">
  <h2 id="team-behind-hyperswitch">Team behind Hyperswitch</h2>
</a>

The core team of 150+ engineers building Hyperswitch. Keep up the great work! 🥂

<a href="https://github.com/juspay/hyperswitch/graphs/contributors">
  <img src="https://contributors-img.web.app/image?repo=juspay/hyperswitch" alt="Contributors"/>
</a>
