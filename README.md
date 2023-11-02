# Overview


## rust-tls-microservice

This is a simple microservice used as a basis for the UPaaS project (Unikernel base Platform As A Service)

### Usage

Clone this repo

```
cd rust-tls-microservice

cargo build
```

Execute the binary

```
# use tls (see the section below to create self signed certs)
# create a directory called certs and copy all certs this directory
# name your certs 'domain' 
./target/release/rust-tls-microservice -t 

# default is without tls
./target/release/rust-tls-microservice 
```


### Creating self signed Certs 

- Create private key

```
openssl genrsa -out domain.key 2048
```

- Create a certifcate signing request

```
openssl req -key domain.key -new -out domain.csr
```

OR 

with one command

```
openssl req -newkey rsa:2048 -nodes -keyout domain.key -out domain.csr
```

- Create a self signed certifcate

```
openssl req -newkey rsa:2048 -keyout domain.key -x509 -days 365 -out domain.crt
```

- Create a CA signed cert with our own CA

```
openssl req -x509 -sha256 -days 1825 -newkey rsa:2048 -keyout rootCA.key -out rootCA.crt
```

- Sign our CSR with root CA

- First create a text-file with the following content

```
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
subjectAltName = @alt_names
[alt_names]
DNS.1 = <your-domain>
```

- Sign the domain csra

```
openssl x509 -req -CA rootCA.crt -CAkey rootCA.key -in domain.csr -out domain.crt -days 365 -CAcreateserial -extfile domain.ext
```

- Convert formats if needed 

```
openssl x509 -in domain.crt -outform der -out domain.der

openssl pkcs12 -inkey domain.key -in domain.crt -export -out domain.pfx
```
