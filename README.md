## health check

**URI:** http://localhost:8000 </br>
**METHOD:** GET </br>

## new transaction

**URI:** http://localhost:8000/new_transaction </br>
**METHOD:** POST </br>
**BODY:**

```json
{
  "from": "public key? user name?",
  "to": "<optional for now>",
  "content": "json content"
}
```

O `body` que é o `dado` de cada transação é salvo na chain em forma de String (stringified JSON).

## create account

**URI:** http://localhost:8000/create_account </br>
**METHOD:** POST </br>
**BODY:**
**AUTO-MINE:** true

```json
{
  "account": "wendersonpires.kib"
}
```

Esta rota faz executa o `mine` automaticamente pois deve-se assegurar que uma conta igual não seja registrada novamente.

## mine

**URI:** http://localhost:8000/mine </br>
**METHOD:** GET </br>

## chain

**URI:** http://localhost:8000/chain </br>
**METHOD:** GET </br>

Durante o processo de buscar os blocos, todos os dados (`transactions` / `dado`) é convertido para JSON novamente para ser exibido ao cliente final.

## Obs

Creio que a ação de `mine` tenha que está do lado do cliente de alguma forma.

## Notes

(Usa esse metodo abaixo para criar conta / assinar transacao)
Private / Public key (Flow para entendimento):

- Site pra testar a ideia: https://www.devglan.com/online-tools/rsa-encryption-decryption

1 - Bob gera sua private key | public key
2 - Bob pode compartilhar seu public key
3 - Alice faz o mesmo, (ela tem sua private e public key)
4 - Bob envia uma mensagem para Alice usando a public key dela para encriptografar a mensagem.
5 - Alice usa a private key dela para descriptografar a mensagem enviada por Bob.
