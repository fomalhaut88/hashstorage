# hashstorage

## Description

**Hashstorage** is a web service that provides an opportunity to upload and modify data keeping its ownership (using signatures according to [ECDSA](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm)). Briefly, everybody who has a private key can put data into a Hashstorage instance but nobody can modify the data silently, even the owner of the instance. Also everybody can read the data (if it is not encrypted). Particularly, it is allowed to encrypt the put data (with, for example, [AES-256](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard) algorithm) before saving, so everybody can read this record in the Hashstorage instance, but nobody can understand and modify the original phrase. Thus Hashstorage can be a fully secure cloud service. It is a perfect solution if the clients are supposed not to trust the server side.

Hashstorage uses 256-bit ECDSA over [Secp256k1](https://en.bitcoin.it/wiki/Secp256k1) (Bitcoin curve).

Hashstorage is written in [Actix Web](https://github.com/actix/actix-web) to implement the REST API and to provide high performance. Also it includes [bigi-ecc](https://github.com/fomalhaut88/bigi-ecc) as the library to work with the cryptography issues inside.


## Client side

To work with a Hashstorage instance from the client side there is a JS library [hashstorage-cli](https://github.com/fomalhaut88/hashstorage-cli) that contains all the necessary methods to create key pairs, sign data, construct blocks to save, etc.


## How it works

In Hashstorage, data are stored as key-value records (fields **key** and **data**), that have their owners (field **public**), signatures (field **signature**) and versions (field **version**). Data records can be grouped (field **group**). Group and version can be empty strings.

Fields of a record:

* `public` - the owner of the record, 256-bit public key as a point on Secp256k1.
* `group` - the group (it can be empty, the length is up to 32 characters).
* `key` - key of the data, any string (it can be empty, the length is up to 32 characters).
* `version` - the version of the data (unsigned 64-bit integer).
* `data` - value of the data, any string up to 16 MB.
* `signature` - ECDSA signature built from concatenated `group`, `key`, `version` and data hashed with [SHA-256](https://en.wikipedia.org/wiki/SHA-2) algorithm.

The database has a unique key: `(public, group, key)`.

### Generating keys

On the client side, before working with records, it is necessary to generate a key-pair on Secp256k1. Private key should be stored reliably somewhere, public key will be used in data records. Hashstorage will check the ownership with the help of the public key. Private key is needed to generate signatures. This procedure can be done with `generate_pair` in [hashstorage-utils](https://github.com/fomalhaut88/hashstorage-utils).

### Creating a signature

First, it is necessary to concatenate **group**, **key**, **version** and **data** into a single byte sequence. After that, SHA-256 algorithm must be applied to it, so there will be a 256-bit hash as the result. In the end, a signature can be created from this hash using ECDSA algorithm. Hashstorage checks the signature on save. This procedure can be done with `build_signature` in [hashstorage-utils](https://github.com/fomalhaut88/hashstorage-utils).

### Big number format

All the numbers (private and public keys, signatures, secret, etc) must be in upper cased HEX format without leading 0x. Here is an example of a valid private key:

    12BEC995D37D5267AD734B5B63FFFF048A511F71CD086D3E212FF13C9A037FD1

### Version

Hashstorage instance checks the given version on update a record, its value should be strictly greater than the old one. Otherwise, the service returns the error `412 Precondition Failed invalid version`. This is done in order to prevent any possibility of rolling back the data on the server side stealthily by the owner of the instance. As far as the data records are and were signed with the signatures, it is impossible to put a completely new record with he correct signature. But it is still possible to repeat one of the previous requests to set a previous state of the record (with the right signature). Using incrementing versions on each save and storing the value of the last version on the client side, it is easy to detect if the remote version has been changed (decremented) by somebody.


## Advantages and disadvantages of Hashstorage comparing to ordinary cloud services

Advantages:
- each record has a proof of client ownership,
- the owner of a Hashstorage instance is unable to modify the data,
- the owner of a Hashstorage instance is unable to read, analyze and share with 3rd parties the records if they are encrypted,
- the same Hashstorage instance can be easily and naturally used for multiple projects,
- no need to use SSL usually (HTTPS is acceptable, but usually it is not necessary, because everything is already encrypted well enough; although everything depends on the sort of the data in the project).

Disadvantages:
- relatively complicated way to manage the data.


## Areas to use

- Storing private data of the clients (passwords, private notes, location, financial details, etc),
- Publishing open information with its ownership.


## Recommendations to use

### Generating a key-pair by username and password within a custom application

As far as a private key is a 256-bit number, it can be some hash of username, password, project ID or any other data that can identify the client in a custom application. Simply it is possible to use SHA-256 hash for it. The public key can be generated according to ECC on Secp256k1 as a multiplication on the generator. One of the recommended ways to hash the data is following:

    SHA256({APP_UUID}:{USERNAME}:{PASSWORD})

Where **APP_UUID** is a unique identifier of the application (it can be [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier), for example), you should use it if you have multiple applications that work with the same Hashstorage instance. The password should contain as least 12 symbols with upper and lower cases and digits.

### Data encryption

To encrypt data before sending it to Hashstorage, it is a good idea to encrypt it with AES-256 using the private key as a password. It is enough because nobody except for the client knows the private key, so nobody can decrypt the data (unless AES-256 algorithm is broken, of course).

### Signatures

The signatures must be generated according to ECDSA, keeping the number format described above.


## Methods

| URL | Method | Description | Request example | Response example |
|---|---|---|---|---|
| `/version` | GET | Version of the Hashstorage instance. | | ```{"version":"2.2.0"}``` |
| `/groups/{public}` | GET | List of groups related to given public key `{public}`. | `/groups/F97CF0EA9BA1C36BE29045A14AAC32ED9ECD8D67A9D6823D623E161B2600ED3B4D3FA95A1580FED6068BD67013C990524DCCE132350EAC38948E3E15BC3E1E60` | `["mygroup"]` |
| `/keys/{public}/{group}` | GET | List of keys related to given public key `{public}` and group `{group}`. | `/keys/F97CF0EA9BA1C36BE29045A14AAC32ED9ECD8D67A9D6823D623E161B2600ED3B4D3FA95A1580FED6068BD67013C990524DCCE132350EAC38948E3E15BC3E1E60/mygroup` | `["mykey"]` |
| `/info/{public}/{group}/{key}` | GET | Get information about the stored block: all fields except for `data`. | `/data/F97CF0EA9BA1C36BE29045A14AAC32ED9ECD8D67A9D6823D623E161B2600ED3B4D3FA95A1580FED6068BD67013C990524DCCE132350EAC38948E3E15BC3E1E60/mygroup/mykey` | `{"signature":"5075084839ED79A54B3C08BAA0F236E48737C835C0CA2622F647BFD128BABE7C5E93A4BC020C89F7FA5A232A559D7C148C01CEBE13EEC3F77640E4FE9D748305","public":"F97CF0EA9BA1C36BE29045A14AAC32ED9ECD8D67A9D6823D623E161B2600ED3B4D3FA95A1580FED6068BD67013C990524DCCE132350EAC38948E3E15BC3E1E60","group":"mygroup","key":"mykey","version":1}` |
| `/data/{public}/{group}/{key}` | GET | Get all fields of the stored block. | `/data/F97CF0EA9BA1C36BE29045A14AAC32ED9ECD8D67A9D6823D623E161B2600ED3B4D3FA95A1580FED6068BD67013C990524DCCE132350EAC38948E3E15BC3E1E60/mygroup/mykey` | `{"signature":"5075084839ED79A54B3C08BAA0F236E48737C835C0CA2622F647BFD128BABE7C5E93A4BC020C89F7FA5A232A559D7C148C01CEBE13EEC3F77640E4FE9D748305","public":"F97CF0EA9BA1C36BE29045A14AAC32ED9ECD8D67A9D6823D623E161B2600ED3B4D3FA95A1580FED6068BD67013C990524DCCE132350EAC38948E3E15BC3E1E60","group":"mygroup","key":"mykey","version":1,"data":"Hello world"}` |
| `/data/{public}/{group}/{key}` | POST | Save or update the data block. The body must be a JSON. | `/data/F97CF0EA9BA1C36BE29045A14AAC32ED9ECD8D67A9D6823D623E161B2600ED3B4D3FA95A1580FED6068BD67013C990524DCCE132350EAC38948E3E15BC3E1E60/mygroup/mykey` | |


## How to deploy Hashstorage from source

### 1. Clone the repository

```
git clone https://github.com/fomalhaut88/hashstorage.git --depth 1
cd hashstorage
```

### 2. Configure the instance

Open `start.sh` and modify:

1. `HASHSTORAGE_PORT` to set the desirable port.
2. `HASHSTORATE_DB_DIR` to set the desirable database location.

### 3. Run the instance in Docker

To start: `sudo ./start.sh`

To stop: `sudo ./stop.sh`

To restart: `sudo ./restart.sh`

### 4. Check

```
curl localhost:8080/version
```

The version of the service must be shown.


## Projects that use Hashstorage

Qnote - cloud based browser application to make notes: https://github.com/fomalhaut88/qnote

Domestic - an application to manage housework: https://github.com/fomalhaut88/domestic
