# ValidateIT API - Terms of Service

## 1. ACCEPTANCE OF TERMS

By accessing or using ValidateIT API (the "Service"), you agree to be bound by these Terms of Service ("Terms"). If you do not agree to these Terms, do not use the Service.

## 2. SERVICE DESCRIPTION

ValidateIT API provides algorithmic validation services for Italian documents, including but not limited to:
- Italian Fiscal Codes (Codice Fiscale)
- Italian VAT Numbers (Partita IVA)
- Italian and European IBANs

The Service performs validations based on:
- Format checks
- Mathematical checksum algorithms
- Verification against public reference databases

**IMPORTANT:** The Service is NOT:
- An official identity verification service
- An official authentication system
- A source of personal or registry data
- A substitute for professional or legal verification

## 3. INFORMATIONAL NATURE OF SERVICE

### 3.1 Limited Purpose

The Service is provided **solely for informational and technical support purposes**. Validations provided are based on publicly documented algorithms and do NOT constitute:

- Official verification with government registries
- Confirmation of actual existence of the person/company
- Validation of active/inactive status of the document
- Verification of document-identity correspondence
- Fiscal, legal, or professional advice of any kind

### 3.2 No Substitute for Professionals

ValidateIT API does **NOT replace**:
- Accountants
- Tax advisors
- Lawyers
- Notaries
- Auditors
- Government tax agencies
- Any other qualified professional

For legal, tax, or matters requiring official certifications, **always consult a qualified professional**.

## 4. PRIVACY AND DATA PROCESSING

### 4.1 Data NOT Stored

**ValidateIT API does NOT save, store, or retain:**

- Documents sent for validation (CF, VAT, IBAN)
- Personal data extracted from validations
- Identifying information of end users
- Logs containing sensitive data

**What IS stored** (for technical purposes only):
- Anonymous metadata: timestamps, request type, outcome (valid/invalid)
- Aggregate metrics: request count, performance, error rate
- Technical logs: caller IP address (security/debugging only, 30 days max)

### 4.2 GDPR Compliance

The Service is designed to be **GDPR-compliant by design**:
- No processing of personal data beyond immediate validation
- No data transfers to third parties
- No user profiling
- Right to be forgotten is automatic (data not stored = already forgotten)

**GDPR Responsibility:** The Service user (developer/company) is the **data controller** for data collected in their own application. ValidateIT API acts as a **limited data processor** only during request processing.

### 4.3 User Obligations

Users agree to:
- Inform their end users about the use of ValidateIT API
- Obtain appropriate consent for data collection/processing
- Not transmit unnecessary sensitive data
- Implement security measures in their own application
- Comply with all applicable privacy regulations

## 5. LIMITATIONS OF LIABILITY

### 5.1 Data Accuracy

**ValidateIT API is provided "AS IS" without warranties of any kind.**

We strive to maintain the Service accurate and up-to-date, but we do **NOT guarantee:**

- **100% accuracy:** Algorithms may contain errors or be superseded by regulatory changes
- **Up-to-date databases:** Reference databases (municipalities, ABI, CAB) may not be updated in real-time
- **Complete coverage:** There may be edge cases or valid documents not recognized
- **Absence of false positives/negatives:** Valid documents may be rejected or vice versa

**Examples of possible inaccuracies:**
- Municipalities merged/suppressed after last database update
- Banks with recently modified ABI codes
- Edge cases not covered in algorithms
- Errors in public reference databases

### 5.2 Service Availability

We do **NOT guarantee:**
- 100% uptime (target: 99.9% but not binding for FREE/STARTER tiers)
- Absence of scheduled maintenance interruptions
- Consistent response times
- Availability during extraordinary events (DDoS attacks, natural disasters, etc.)

### 5.3 Indirect Damages

**ValidateIT API and its operators are NOT liable for:**

- Direct or indirect damages arising from use of the Service
- Financial losses caused by incorrect validations
- Tax/legal penalties arising from data not validated correctly
- Reputational damages
- Loss of profits or business opportunities
- User service interruptions caused by API malfunctions
- Any consequential damages from use or inability to use the Service

**Maximum liability:** In any case, ValidateIT API's maximum liability is limited to the amount paid by the user in the last 12 months.

### 5.4 Misuse

ValidateIT API is **NOT responsible** for:

- Fraudulent use of the Service by third parties
- Validations performed for illegal purposes
- Business decisions based solely on API output
- Incorrect implementations by users
- Failure to perform additional validation by users
- Bypassing or manipulation of API responses

## 6. ACCEPTABLE USE

### 6.1 Prohibited Activities

It is **PROHIBITED** to use the Service for:

- **Scraping or reverse engineering:** Attempting to extract algorithms or databases
- **Service abuse:** Exceeding rate limits, DDoS attacks, brute force
- **Illegal activities:** Tax fraud, money laundering, identity theft
- **Spam or phishing:** Mass validation of stolen data
- **Unauthorized resale:** Repackaging API without written permission
- **Direct competition:** Creating competing service using our data
- **Privacy violations:** Collecting personal data without consent

### 6.2 Violation Consequences

In case of violation:
- Immediate account suspension without notice
- Termination of API access without refund
- Reporting to competent authorities (if applicable)
- Legal action for damages incurred

## 7. PLANS AND PAYMENTS

### 7.1 Free Tier

- 100 requests/month free
- No guarantee of availability or support
- May be modified or removed at any time
- Not for production-critical use

### 7.2 Paid Tiers

- Rates according to pricing published on RapidAPI
- Monthly or annual billing
- No refunds for services already provided
- Prices subject to change with 30 days notice

### 7.3 Cancellation

- User may cancel at any time
- Cancellation effective at end of already-paid period
- No pro-rata refunds
- User data deleted within 30 days

## 8. INTELLECTUAL PROPERTY

### 8.1 API Code (MIT License)

ValidateIT API source code is released under MIT License (see LICENSE file).

**This means you can:**
- Use the code for any purpose
- Modify the code
- Distribute copies of the code
- Use the code in commercial projects

**Provided that:**
- You include the copyright notice and MIT License
- You accept the code is provided "AS IS" without warranties

### 8.2 API Service (Proprietary)

The **hosted service** ValidateIT API, including:
- Infrastructure
- Curated proprietary databases
- "ValidateIT" name and brand
- API documentation

Remain the exclusive property of ValidateIT and are subject to these Terms of Service.

### 8.3 Reference Databases

The databases used (municipalities, ABI/CAB) derive from public sources (ISTAT, Bank of Italy) and are subject to their respective usage licenses.

## 9. MODIFICATIONS TO TERMS

ValidateIT reserves the right to modify these Terms at any time.

**Procedure:**
- Email notification 30 days in advance
- Publication of new terms on website
- Continued use = acceptance of new terms
- Disagreement = right to cancel account

## 10. GOVERNING LAW AND JURISDICTION

### 10.1 Italian Law

These Terms are governed by **Italian law**.

### 10.2 Jurisdiction

For any dispute, the **Court of [YOUR CITY]**, Italy has exclusive jurisdiction.

### 10.3 Amicable Resolution

Before proceeding legally, parties agree to attempt amicable resolution of disputes through direct negotiation (60 days).

## 11. MISCELLANEOUS PROVISIONS

### 11.1 Severability

If any provision is found invalid, the remaining provisions remain in effect.

### 11.2 Waiver

Failure to enforce any provision does not constitute waiver.

### 11.3 Entire Agreement

These Terms constitute the entire agreement between parties, superseding any prior agreements.

### 11.4 Assignment

User may not assign rights/obligations without written consent. ValidateIT may assign to third parties.

---

By using ValidateIT API, you acknowledge that you have read, understood, and agree to be bound by these Terms of Service.
