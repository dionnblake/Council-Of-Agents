# Public Repository Data Policy

Council of Agents may publish architecture, provider versions, generalized paths such as `C:\Users\<USER>` or `/home/<USER>`, hashes of public or synthetic fixtures, reproducible commands using placeholders, and pass/fail verification results.

Public engineering evidence must not contain:

- personal workstation hostnames or private usernames;
- private Windows or Linux home paths;
- actual credentials, API keys, tokens, private-key material, or bearer values;
- credential prefixes or lengths when they are not needed to explain the engineering result;
- private account identifiers or personal configuration inventories;
- unrelated personal, employment, financial, health, family, or address information.

Before a public push, run:

```text
node scripts/public-repo-audit.cjs --history
```

The check is local-only and reports finding categories and repository-relative locations without printing matched values. A potential credential is treated as compromised until removed from the current tree and reachable history; required rotation or revocation happens manually outside the repository.
