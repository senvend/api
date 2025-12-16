# PayService API Endpoint

#### Endpoint address

`local.v1.PayService`

#### Defined methods

- `Pay (stream api.v1.PayRequest) returns (stream api.v1.PayResponse)`  
  Accepts a stream of PayRequest for starting and controlling payments.  
  Returns a stream of PayResponse containing status and error return messages.

## Message Flow

#### API Constraints

- Request ids are optional.  
  If none is given and a process is running, the request is applied to that running process.  
  Otherwise a new uuid is generated per request.

- If request ids are pre-generated and part of the request, subsequent requests meant for that process have to use the same uuid. (Or leave the id field empty)

- If disconnected during a payment process, after a reconnect the currently running process can still be controlled.  
  PayResponses that occurred during the disconnect are lost though.

#### Process Constraints

- The amount to charge is given in cents and has to be greater than zero.

- The minimum age to verify has to be greater than zero and can maximally be 120.  
  Depending on the method chosen, only certain ages can be verified.

- If a payment was approved, a GoodsIssued message must be sent in order to finalize it.  
  The client has 9m50s to answer to the approval, or the payment will be reimbursed to the customer as an emergency measure.

#### State and state changes

| State | Request | Result |
|--------------------------|----------------------------|------------------------------|
| No payment running | PayStart | Payment start |
| | PayStart (with AgeRequest) | AgeVerification start |
| | PayCancel | ApiError |
| | PayGoodsIssued | ApiError |
| Age verification ongoing | PayStart | ApiError |
| | PayCancel | AgeVerification cancel |
| | PayGoodsIssued | ApiError |
| Payment process ongoing | PayStart | ApiError |
| | PayCancel | Payment cancel |
| | PayGoodsIssued | ApiError |
| Payment accepted | PayStart | ApiError |
| | PayCancel | Reimburse and cancel payment |
| | PayGoodsIssued | Finalize payment |

#### State and errors

| State | Event | Error |
|--------------------------|---------------------------|-----------------------------------|
| Age verification ongoing | Took too long (timeout) | AGE_FAILURE_REASON_USER_CANCELLED |
| | User actively canceled | AGE_FAILURE_REASON_USER_CANCELLED |
| | Cancel via api | AGE_FAILURE_REASON_API_CANCELLED |
| | Verification fails | AgeFailureUnderage |
| Payment process ongoing | Took too long (timeout) | PAY_FAILURE_REASON_USER_CANCELLED |
| | User actively canceled | PAY_FAILURE_REASON_USER_CANCELLED |
| | Canceled via api | PAY_FAILURE_REASON_API_CANCELLED |
| | Payment failed (no debit) | PAY_FAILURE_REASON_PAYMENT_FAILED |
| Payment accepted | No response from api | PaySuccess |

## Examples

For more information on how to read and run these examples:  
[API examples and test tools](./ApiBasics.md#Examples)

### Full payment process with age verification (no errors)

> **->** {"start": {"amount": 100, "age_verification": {"min_age": 18}}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageApiSuccess": { "reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED" }}

- *Age Verification on device* (success)

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "ageSuccess": {}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (success)

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "approved": {}}

> **->** {"goods_issued":{}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED"}}

> **\<-** {"id": {"msb": "10249154777407571789", "lsb": "11282912518529581516"}, "success": {}}

### Payment process without age verification (no errors, using pre-generated uuids)

> **->** {"id":{"msb":5, "lsb":0}, "start": {"amount": 100}}

> **\<-** {"id": {"msb": "5"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (success)

> **\<-** {"id": {"msb": "5"}, "approved": {}}

> **->** {"id": {"msb":5, "lsb":0}, "goods_issued": {}}

> **\<-** {"id": {"msb": "5"}, "apiSuccess": { "reason": "PAY_API_SUCCESS_REASON_GOODS_ISSUED_ACCEPTED" }}

> **\<-** {"id": {"msb": "5"}, "success": {}}

### Payment process without age verification (payment failed)

> **->** {"start": {"amount": 220}}

> **\<-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "apiSuccess": {"reason": "PAY_API_SUCCESS_REASON_PAYMENT_STARTED"}}

- *Payment on device* (failed)

> **\<-** {"id": {"msb": "3750298859991747092", "lsb": "10323224318664740260"}, "failure": {"failureReason": "PAY_FAILURE_REASON_PAYMENT_FAILED"}}

### Full payment process with age verification (age verification failed)

> {"start": {"amount": 100, "age_verification": {"min_age": 18}}}

> **\<-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageApiSuccess": {"reason": "AGE_API_SUCCESS_REASON_VERIFICATION_STARTED"}}

- *Age Verification on device* (failed)

> **\<-** {"id": {"msb": "2610397151102190428", "lsb": "11915408413712218260"}, "ageFailure": {"underAge": {}}}
