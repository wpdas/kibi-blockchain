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

## mine

**URI:** http://localhost:8000/mine </br>
**METHOD:** GET </br>

## chain

**URI:** http://localhost:8000/chain </br>
**METHOD:** GET </br>

Durante o processo de buscar os blocos, todos os dados (`transactions` / `dado`) é convertido para JSON novamente para ser exibido ao cliente final.

## Obs

Creio que a ação de `mine` tenha que está do lado do cliente de alguma forma.

## Onde Parei

Parei tentando fazer a funçao de mine se tornar async (mas vale a pena? ja que essa funcao deve ser do client?)
