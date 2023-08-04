# ValidationC2B API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Initiate ValidationC2B

```sh
curl -d '{"TransactionType": "Pay Bill","TransID":"RKTQDM7W6F4","TransTime":"20191122063845","TransAmount":"1500","BusinessShortCode": "6***38","BillRefNumber":"KBA 8**J","InvoiceNumber":"","OrgAccountBalance":"","ThirdPartyTransID": "","MSISDN":"hashed","FirstName":"John","MiddleName":"","LastName":"Doe"}' -H 'Content-Type: application/json' http://localhost:8080/validationc2b

http POST :8080/validationc2b TransactionType="Pay Bill" TransID="RKTQDM7W6F4" TransTime="20191122063845" TransAmount="1500" BusinessShortCode="6***38" BillRefNumber="KBA 8**J" InvoiceNumber":"" OrgAccountBalance="" ThirdPartyTransID="" MSISDN="hashed" FirstName="John" MiddleName="" LastName="Doe"
```

The response should be a 200 OK with the following JSON body:

```json
{
  "ResultCode": "0",
  "ResultDesc": "Accepted"
}
```
