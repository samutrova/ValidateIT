# ValidateIT - Italian Document Verification

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> Lightning-fast Italian document validation API built with Rust

## Problem

Every Italian developer wastes 20-40 hours implementing CF/PIVA/IBAN validations that then break when municipalities or algorithms change. We provide a battle-tested API that solves the problem in 5 minutes, always up to date, for €9/month.

**Start the free trial** [ValidateIT](https://rapidapi.com/samutrovarelli/api/validateit-italian-document-validation)

## Examples

### Example Requests

#### Request

```GET /cf/{cf}/validate```

Bash
```bash
curl -X GET "https://validateit-italian-document-validation.p.rapidapi.com/cf/{cf}/validate" -H "x-rapidapi-key: <your-api-key>" -H "x-rapidapi-host: validateit-italian-document-verification.p.rapidapi.com"
```

Javascript
```javascript
const cf = "RSSMRA85M01H501Z";
const url = `https://validateit-italian-document-validation.p.rapidapi.com/cf/${cf}/validate`;

const options = {
  method: 'GET',
  headers: {
    'x-rapidapi-key': '<your-api-key>',
    'x-rapidapi-host': 'validateit-italian-document-validation.p.rapidapi.com'
  }
};

fetch(url, options)
  .then(response => response.json())
  .then(data => console.log(data))
  .catch(error => console.error('Errore:', error));
```

Php
```php
<?php

$cf = "RSSMRA85M01H501Z";
$url = "https://validateit-italian-document-validation.p.rapidapi.com/cf/" . urlencode($cf) . "/validate";

$ch = curl_init();

curl_setopt($ch, CURLOPT_URL, $url);
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
curl_setopt($ch, CURLOPT_HTTPHEADER, [
    'x-rapidapi-key: <your-api-key>',
    'x-rapidapi-host: validateit-italian-document-validation.p.rapidapi.com.p.rapidapi.com'
]);

$response = curl_exec($ch);

if (curl_error($ch)) {
    echo 'Errore cURL: ' . curl_error($ch);
} else {
    $data = json_decode($response, true);
    print_r($data);
}

curl_close($ch);
?>
```

Python
```python
import requests

cf = "RSSMRA85M01H501Z"
url = f"https://validateit-italian-document-validation.p.rapidapi.com/cf/{cf}/validate"

headers = {
    "x-rapidapi-key": "<your-api-key>",
    "x-rapidapi-host": "validateit-italian-document-validation.p.rapidapi.com"
}

response = requests.get(url, headers=headers)

if response.status_code == 200:
    print(response.json())
else:
    print(f"Errore {response.status_code}: {response.text}")
```

#### Basic Plan Response

```json
{
  "valid": false,
  "type": "fiscal_code",
  "message": "Fiscal Code checksum is not valid",
  "checks": {
    "format": true,
    "checksum": false
  }
}
```

#### Pro Plan Response

```json
{
  "valid": false,
  "type": "fiscal_code",
  "data": {
    "fiscal_code": "RSSMRA85M01H501Z",
    "surname": "RSS",
    "name": "MRA",
    "gender": "M",
    "date_of_birth": "1985-08-01",
    "place_of_birth": {
      "name": "Roma",
      "cadastral_code": "H501",
      "province": "RM"
    },
    "check_digit": "Z"
  },
  "errors": [
    {
      "field": "check_digit",
      "message": "Invalid check digit: expected Q, found Z",
      "status_code": 400
    }
  ]
}
```

More examples on RapidAPI
[More Examples](https://rapidapi.com/samutrovarelli/api/validateit-italian-document-validation/playground/apiendpoint_daa9699c-cc22-4307-8149-4b7025ef2068)

## Plans

### Basic Plan

- **0 €/month**
- **100 req/month**
- **CF/PIVA/IBAN format and checksum validation**

### Pro Plan

- **9 €/month**
- **5,000 req/month**
- Basic + **CF/PIVA/IBAN extraction**

### Ultra Plan

- **19 €/month**
- **20,000 req/month**

### Mega Plan

- **49 €/month**
- **100,000 req/month**

[See our plans for details](https://rapidapi.com/samutrovarelli/api/validateit-italian-document-validation)

## Issues

If you encounter some bugs let we know!
[Report Problems](https://rapidapi.com/samutrovarelli/api/validateit-italian-document-validation/discussions)

## Legal & Compliance

### License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

### Terms

- **[Terms of Service](TERMS_OF_SERVICE.md)** - Please read before using the API
- **[Disclaimer](DISCLAIMER.md)** - Important limitations and responsibilities

### GDPR Compliance

ValidateIT API is **GDPR-compliant by design**:

**No storage** of validated documents (CF, VAT, IBAN)  
**No personal data retention** beyond technical metadata  
**EU-only data hosting** (Google Cloud EU)  
**Full data subject rights support**  
**Minimal data collection** (only what's necessary for service operation)  
**Transparent processing** - see Privacy Policy for details

### Data Processing Summary

| What We Validate | What We Store | Retention |
|------------------|---------------|-----------|
| Fiscal Codes | Nothing | 0 seconds |
| VAT Numbers | Nothing | 0 seconds |
| IBANs | Nothing | 0 seconds |
| Request metadata | IP, timestamp | 30 days (security) |
| Usage metrics | Aggregated only | Indefinite |

### Your Responsibilities

When using ValidateIT API, you are responsible for:

- **Compliance** with applicable laws in your jurisdiction
- **Proper implementation** in your application
- **User consent** for data processing
- **Security measures** in your application
- **Professional validation** where legally required
- **Privacy notices** to your end users

### Important Disclaimers

**This is an informational service only:**

- Does NOT replace official government verification
- Does NOT guarantee 100% accuracy
- Does NOT constitute professional advice
- May contain errors or outdated information

For legal, tax, or financial matters, **always consult a qualified professional**.

### Security & Privacy

**What we do:**
- HTTPS/TLS 1.3 encryption
- In-memory validation only
- No permanent storage of sensitive data
- Regular security audits
- Rate limiting and abuse prevention

**What we don't do:**
- Store your validated documents
- Share data with third parties
- Profile or track end users
- Use data for marketing

---

## Acceptable Use

**Permitted:**
- Validating documents for your application
- Form validation and data quality checks
- Integration in commercial products
- Reasonable automated usage within rate limits

**Prohibited:**
- Scraping or reverse engineering
- Exceeding rate limits / abuse
- Illegal activities (fraud, money laundering, etc.)
- Mass validation of stolen data
- Creating competing services without permission

Violations may result in immediate account suspension and legal action.

---

## Compliance Certifications

- **GDPR Compliant** - EU data protection standards
- **ISO 27001** (planned) - Information security management
- **SOC 2** (planned) - Service organization controls

---

By using ValidateIT API, you agree to our [Terms of Service](TERMS_OF_SERVICE.md) and acknowledge our [Disclaimer](DISCLAIMER.md).

### Usage Responsibility

You are solely responsible for:
- Compliance with applicable laws in your jurisdiction
- Proper implementation in your application
- Obtaining necessary consents from your end-users
- Professional validation where legally required

## Links

- [ValidateIT](https://rapidapi.com/samutrovarelli/api/validateit-italian-document-validation)
- [License](LICENSE)
- [Issues](https://rapidapi.com/samutrovarelli/api/validateit-italian-document-validation/discussions)
- [GitHub](https://github.com/samutrova/ValidateIT)

- **[Terms of Service](TERMS_OF_SERVICE.md)** - Please read before using the API
- **[Disclaimer](DISCLAIMER.md)** - Important limitations and responsibilities

---

**Made with ❤️ in Italy**
