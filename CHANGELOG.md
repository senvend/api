# CHANGELOG

## v1.0.0 (2026-09-15)

- First mainline release of the SENVEND public API.
  - APPROVED PayResponse now contains the actual approved amount.
    Can be used for verification during PayUpdate.
  - Vending service. Can be used standalone or during the payment process to vend items via MDB.
  - Free vending (amount=0 and cashAmount=0) is now allowed.
    This enables using the PayService for any workflow while keeping telemetry related to sales.
    E.g. only age verification or only vending.
  - Example clients for Rust, Python, Kotlin and C#, plus a wrapper script for buf curl.
  - Client libraries are published as `senvend_api` on crates.io, as `senvend-api` / `senvend-api-async` on PyPI, as `com.senbax.senvend:senvend-api` on Maven Central and as `Senvend.Api` on NuGet.
  - All protobuf definitions are published to the SENVEND schema registry on buf.build.

## v0.2.0 (2026-06-01)

- Additional features added to the SENVEND public API.
  - PayUpdate message. Can change the charged amount after PayStart but before the APPROVED response.
    Mostly useful for mixed payments if the API client manages cash itself.
  - Cash amount added to all messages with an amount (PayStart, PayUpdate, GoodsIssued).
    Used for telemetry reporting.
  - Payments with amount=0 are now allowed via PayStart and PayUpdate.
    This enables cash-only payment while still sending sale telemetry.
  - AgeVerification can now be approved by the API client, both in the AgeVerificationService and the PayService.
    Useful if the client has additional age verification methods it handles itself.

## v0.1.0 (2026-02-05)

- First release of the SENVEND public API.
  - Accessible via local (network) connection to the SENVEND Terminal. With or without TLS and/or password.
  - Age verification service. End users can choose between all configured age verification methods on the device screen.
  - Payment service with optional age verification before payment.
  - Payments can be tracked via the SENVEND web portal, including quantity and price of sold products if provided via the API.
  - Version Service to check which API features are supported on the device.
