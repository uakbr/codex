# Authentication

## Usage-based billing alternative: Use an OpenAI API key

If you prefer to pay-as-you-go, you can still authenticate with your OpenAI API key:

```shell
printenv OPENAI_API_KEY | codex login --with-api-key
```

Alternatively, read from a file:

```shell
codex login --with-api-key < my_key.txt
```

The legacy `--api-key` flag now exits with an error instructing you to use `--with-api-key` so that the key never appears in shell history or process listings.

This key must, at minimum, have write access to the Responses API.

## Migrating to ChatGPT login from API key

If you've used the Codex CLI before with usage-based billing via an API key and want to switch to using your ChatGPT plan, follow these steps:

1. Update the CLI and ensure `codex --version` is `0.20.0` or later
2. Delete `~/.codex/auth.json` (on Windows: `C:\\Users\\USERNAME\\.codex\\auth.json`)
3. Run `codex login` again

## Connecting on a "Headless" or Remote Machine

**New in 0.58.0+**: Codex now automatically detects headless and remote environments and uses device code authentication, eliminating the need for manual workarounds in most cases!

### Automatic Headless Detection

When you run `codex login` in a remote or headless environment, Codex automatically detects this and uses **device code authentication** instead of the localhost-based browser flow. This happens automatically when any of these conditions are met:

- SSH session (detected via `SSH_CONNECTION`, `SSH_CLIENT`, or `SSH_TTY` environment variables)
- GitHub Codespaces
- VS Code Remote Containers or Remote Development
- Docker container
- tmux or screen session without a display
- Unix/Linux system without `DISPLAY` or `WAYLAND_DISPLAY`

**Device code authentication flow:**
1. Run `codex login` on your remote machine
2. You'll see a one-time code and a URL (e.g., `https://auth.openai.com/codex/device`)
3. Open that URL in any browser (on any machine)
4. Enter the one-time code shown in your terminal
5. Complete the authentication
6. The terminal session will automatically complete the login

This works seamlessly in Docker containers, SSH sessions, CI environments, and any other headless setup.

### Override Auto-Detection

If you have port forwarding set up and want to force browser-based login even in a headless environment, use:

```shell
codex login --browser
```

Or to explicitly use device code authentication (even if not detected as headless):

```shell
codex login --device-auth
```

### Legacy Workarounds (Pre-0.58.0)

The following workarounds are no longer necessary with automatic detection, but are preserved for reference:

#### Authenticate locally and copy your credentials to the "headless" machine

The easiest solution is likely to run through the `codex login` process on your local machine such that `localhost:1455` _is_ accessible in your web browser. When you complete the authentication process, an `auth.json` file should be available at `$CODEX_HOME/auth.json` (on Mac/Linux, `$CODEX_HOME` defaults to `~/.codex` whereas on Windows, it defaults to `%USERPROFILE%\\.codex`).

Because the `auth.json` file is not tied to a specific host, once you complete the authentication flow locally, you can copy the `$CODEX_HOME/auth.json` file to the headless machine and then `codex` should "just work" on that machine. Note to copy a file to a Docker container, you can do:

```shell
# substitute MY_CONTAINER with the name or id of your Docker container:
CONTAINER_HOME=$(docker exec MY_CONTAINER printenv HOME)
docker exec MY_CONTAINER mkdir -p "$CONTAINER_HOME/.codex"
docker cp auth.json MY_CONTAINER:"$CONTAINER_HOME/.codex/auth.json"
```

whereas if you are `ssh`'d into a remote machine, you likely want to use [`scp`](https://en.wikipedia.org/wiki/Secure_copy_protocol):

```shell
ssh user@remote 'mkdir -p ~/.codex'
scp ~/.codex/auth.json user@remote:~/.codex/auth.json
```

or try this one-liner:

```shell
ssh user@remote 'mkdir -p ~/.codex && cat > ~/.codex/auth.json' < ~/.codex/auth.json
```

#### Manual port forwarding

If you run Codex on a remote machine (VPS/server) and want to use browser-based login, forward port 1455 to your machine before starting the login flow:

```bash
# From your local machine
ssh -L 1455:localhost:1455 <user>@<remote-host>
```

Then, in that SSH session, run `codex login --browser` and select "Sign in with ChatGPT". When prompted, open the printed URL (it will be `http://localhost:1455/...`) in your local browser. The traffic will be tunneled to the remote server.
