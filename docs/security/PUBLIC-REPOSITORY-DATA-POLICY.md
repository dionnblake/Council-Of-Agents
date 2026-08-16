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

Ordinary Git author and committer email metadata is reported separately as `IDENTITY_METADATA_WARNINGS`. It is not treated as a secret or a release-blocking content finding when no credential, private path, or other prohibited content is present. The audit still exits nonzero for every prohibited content or credential category, and it never prints the email value.
