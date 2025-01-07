---
title: OPC UA Certificates
date: 2025-01-06
authorbox: false
sidebar: false
description: OPC UA Certificates
categories:
  - Programming
archives: 2025-01
tags:
  - OPCUA
draft: true
---
Step by step guide - how to generate and use OPC UA Certificates for communication
<!--more-->

If you using NI OPC UA Toolkit, then common and standard way to create certificate is using the Create Certificate VI:

![](assets/create_cert_snippet.png)

both a public (.der) and private (.pem) key files will be created in the default path (Windows): 

```
C:\ProgramData\National Instruments\certstore\opcua
```

### The `.der` extension

DER (**Distinguished Encoding Rules**) is a binary encoding for X. 509 certificates and private keys. Unlike PEM, DER-encoded files do not contain plain text statements such as -----BEGIN CERTIFICATE----- or -----BEGIN RSA PRIVATE KEY-----. DER files are most commonly seen in Java contexts.

DER is the method of encoding the data that makes up the certificate. DER itself could represent any kind of data, but usually it describes an encoded certificate or a CMS (Cryptographic Message Syntax) container, which can be used to protect messages and to store certificates to allow a receiver to build a trust path to a certificate in the receivers' trust store.

The structure of a certificate is described using the ASN.1 data representation language. Abstract Syntax Notation One is **a standard interface description language (IDL) for defining data structures that can be serialized and deserialized in a cross-platform way**. It is broadly used in telecommunications and computer networking, and especially in cryptography.

### The `.pem` extension

PEM (**Privacy Enhanced Mail**) is a method of encoding binary data as a string (also known as ASCII armor). It contains a header and a footer line (specifying the type of data that is encoded and showing begin/end if the data is chained together) and the data in the middle is the base 64 data. In the case that it encodes a certificate it would simply contain the base 64 encoding of the DER certificate. PEM stands for Privacy Enhanced Mail; mail cannot contain un-encoded binary values such as DER directly.

PEM may also encode / protect other kinds of data that is related to certificates such as public / private keys, certificate requests, so the PEM file may contain just about anything including a public key, a private key, or both, because a PEM file is not a standard.

PEM is a Base64, ASCII encoding that is used for X.509 certificates, certificate requests and cryptographic keys. DER is a binary encoding and is much more compact. In our particular case the PEM file contains RSA PRIVATE KEY only.

### Getting information from DER file

You can information about der file with following command:

```
openssl x509 -in Dmitriev.der -text -noout
```

The openssl.exe tool usually installed with OpenSSL, but slightly older version also deployed by NI

Typical locations:

```
v.3.4.0.0: "C:\Program Files (x86)\OpenSSL-Win32\bin\openssl.exe"
v.3.0.9.0: "C:\Program Files\National Instruments\Shared\Skyline\OpenSSL\openssl.exe"
```

Output information about certificate generated with Create Certificate VI:

```
Certificate:
    Data:
        Version: 3 (0x2)
        Serial Number: 1736173871 (0x677be92f)
        Signature Algorithm: sha256WithRSAEncryption
        Issuer: DC=XXXXXXXXXX, CN=NI OPC UA App
        Validity
            Not Before: Jan  6 14:31:11 2025 GMT
            Not After : Jan  5 14:31:11 2029 GMT
        Subject: DC=XXXXXXXXXX, CN=NI OPC UA App
        Subject Public Key Info:
            Public Key Algorithm: rsaEncryption
                Public-Key: (1024 bit)
                Modulus:
                    00:ab:c7:9c:5c:e1:74:02:ae:aa:f5:ec:34:12:37:
                    a4:7e:2e:fb:3b:b3:24:a4:34:4a:66:66:a3:42:8d:
                    32:12:1c:b4:32:22:85:d9:7a:c2:76:31:14:12:db:
                    71:77:fb:c4:98:ee:fd:db:f8:a5:2e:a0:9f:bb:cc:
                    67:0a:74:32:8d:32:22:32:90:bf:77:ba:7f:a4:02:
                    22:0c:cf:48:5c:9a:14:e2:96:3e:74:95:e8:73:3b:
                    a8:49:77:75:8b:fe:6c:f5:b3:fa:40:9f:fc:44:ef:
                    a4:a7:74:b4:74:60:1d:67:fa:b4:33:b9:cb:5c:a3:
                    6e:ee:4b:e1:a8:4e:ea:09:35
                Exponent: 65537 (0x10001)
        X509v3 extensions:
            X509v3 Basic Constraints: critical
                CA:TRUE, pathlen:0
            Netscape Comment: 
                "Generated with Unified Automation UA Base Library using OpenSSL"
            X509v3 Subject Key Identifier: 
                C1:CA:D1:A2:1B:A1:CB:B3:97:7E:D8:4A:F7:51:42:6E:A7:FC:A8:CD
            X509v3 Authority Key Identifier: 
                keyid:C1:CA:D1:A2:1B:A1:CB:B3:97:7E:D8:4A:F7:51:42:6E:A7:FC:A8:CD
                DirName:/DC=BHI6GC9TB3/CN=NI OPC UA App
                serial:67:7B:E9:2F
            X509v3 Key Usage: critical
                Digital Signature, Non Repudiation, Key Encipherment,
                Data Encipherment, Certificate Sign
            X509v3 Extended Key Usage: critical
                TLS Web Server Authentication, TLS Web Client Authentication
            X509v3 Subject Alternative Name: 
                URI:urn:BHI6GC9TB3:NationalInstruments:Dmitriev, DNS:BHI6GC9TB3
    Signature Algorithm: sha256WithRSAEncryption
    Signature Value:
        a5:a2:ad:05:18:00:62:b9:99:3d:8d:4f:10:f7:75:36:4f:9b:
        39:84:34:32:28:ff:ea:d6:92:d2:0b:93:70:fd:c9:c3:c5:f8:
        f3:80:c0:49:e6:94:ff:49:14:2e:9a:15:86:41:d8:f8:e3:95:
        df:99:b8:39:cf:1a:63:02:e4:af:bc:9f:f6:ca:f4:19:ef:49:
        74:ca:ce:64:17:ba:7f:fc:76:14:38:72:ff:97:ae:4e:44:ff:
        78:d0:1f:3d:ff:0f:1a:a4:91:e7:7a:d8:50:6d:ea:69:7f:ee:
        cf:6b:e4:a2:d1:3c:07:7d:1b:b6:50:2f:d5:1c:40:ed:77:2b:
        ed:49
```

As you can see above , it will expire within 4 years.

### UA Expert Certificate generator

command line:

# Certificate Generator

Unified Architecture certificates can be generated from the command-line using the UA Certificate Generator. The [UA Configuration Tool](https://opcfoundation.github.io/UA-.NETStandard/help/ua_configuration_tool.htm) uses this tool internally. Source code is provided with the sample application source-code.

Run the following command from the command-prompt to obtain important help information:

Opc.Ua.CertificateGenerator.exe -?

 

... the following information will be displayed:

-command or -cmd <issue | revoke | unrevoke | install> The action to perform (default = issue).

-storePath or -sp <filepath>         The directory of the certificate store (mandatory, must be writeable).

-applicationName or -an <name>        The name of the application (mandatory).

-applicationUri or -au <uri>         The URI for the appplication (optional).

-subjectName or -sn <DN>           The distinguished subject name, fields seperated by a / (i.e. CN=Hello/O=World).

-organization or -o <name>          The organization (optional).

-domainNames or -dn <name>,<name>      A list of domain names seperated by commas (optional)

-password or -pw <password>         The password for the new private key file (optional).

-issuerKeyFilePath or -ikf <filepath>    The path to the issuer private key file (optional).

-issuerKeyPassword or -ikp <password>    The password for the issuer private key file (optional).

-keySize or -ks  <bits>           The size of key as a multiple of 1024 (default = 1024).

-lifetimeInMonths or -lm <months>      The lifetime in months (default = 60).

-publicKeyFilePath or -pbf <filepath>    The path to the certificate to renew or revoke (a DER file).

-privateKeyFilePath or -pvf <filepath>    The path to an existing private key to reuse or convert.

-privateKeyPassword or -pvp <password>    The password for the private key.

-reuseKey or -rk <true | false>       Whether to reuse an existing public key (default = false).

-ca <true | false>              Whether to create a CA certificate (default = false).

-pem <true | false>             Whether to output in the PEM format (default = PFX).

 

## Examples

Create a self-signed Application Certificate: -cmd issue -sp . -sn MyApp

Create a CA Certificate: -cmd issue -sp . -an MyCA -ca true

Issue an Application Certificate: -cmd issue -sp . -an MyApp -ikf CaKeyFile -ikp CaPassword

Renew a Certificate: -cmd issue -sp . -pbf MyCertFile -ikf CaKeyFile -ikp CaPassword

Revoke a Certificate: -cmd revoke -sp . -pbf MyCertFile -ikf CaKeyFile -ikp CaPassword

Unrevoke a Certificate: -cmd unrevoke -sp . -pbf MyCertFile -ikf CaKeyFile -ikp CaPassword

Convert key format: -cmd convert true -pw newpassword -pvf MyKeyFile -pvp oldpassword -pem true
