![logo](./assets/logo.png)
# Session Service
Микросервис, отвечающий за управление игровыми сессиями.\
Также хранит скины/плащи.

Имплементирует функционал [Yggdrassil](https://minecraft.fandom.com/wiki/Yggdrasil)

## Содержимое
* [Сборка](#сборка)
* [Деплой](#деплой)
* [Настройка](#настройка)
  * [Переменные окружения](#переменные-окружения)
* [Описание эндпоинтов](#эндпоинты)

# Сборка
Микросервис написан на Rust, так что чтобы\
собрать его вам необходимо лишь установить ``cargo`` на ваш ПК,
и прописать следующую команду

```bash
cargo build --release
```

После успешной сборки вы сможете найти артефакт по этому пути ``./target/release/wss_service``.

# Деплой
Команды для деплоя уже есть в нашем [Puff-файле](./puff.yml).

[Узнать подробнее что такое Puff-файл](https://github.com/smokingplaya/puff)

```bash
# Собирает сервис и пушит его в регистр под тегом latest
puff deploy
```

<!-- # Настройка -->

<!-- ## Переменные окружения -->
<!-- ``DATABASE_URL: string`` - URL для подключения к PostgreSQL. -->

# Эндпоинты

## GET ``/``

### Описание
Возвращает запись о этом Authlib сервисе.

## GET ``/minecraftservices/publickeys``

### Описание
Возвращает публичные ключи.

## POST /login

### Описание
Создаёт игровую сессию.

### Тело
```json
{
  "token": "JWT"
}
```

### Ответ
```json
{jwt
  "selectedProfile": "",
  "accessToken": "",
  "serverId": ""
}
```

## GET ``/profile/{username}``

### Описание
Возвращает профиль указанного игрока

> [!NOTE]
> Дальше идут эндпоинты которые мне лень расписывать, могу лишь сказать что они связаны с заходом на сервер игрока.

## POST ``/sessionserver/session/minecraft/join``

## POST ``sessionserver/session/minecraft/hasJoined``

## POST ``sessionserver/session/minecraft/profile/{uuid}``