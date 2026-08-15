# output-position.v1

Version: 1.0.0

Return one JSON object matching schemas/position.schema.json. The schema is authoritative.

Required:

- schema_version = output-position.v1;
- recommendation;
- commitment: WOULD_STAKE, CONDITIONAL, or WOULD_NOT_STAKE;
- 1–7 load-bearing claims;
- risks;
- flip_condition;
- cost_if_wrong;
- reversibility: EASY, COSTLY, or ONE_WAY_DOOR.

Claims use evidence references in path:startLine-endLine form when repository evidence exists. Do not invent citations. For greenfield questions, distinguish provided facts from external assumptions and use an empty evidence list only when the protocol explicitly allows it.

Do not include Markdown fences, extra prose, numeric confidence, vote counts, provider-majority claims, or workflow instructions.

