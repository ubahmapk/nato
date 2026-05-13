#!/usr/bin/env bash
# Creates or updates the IAM policy that allows the Caddy container to complete
# Route53 DNS-01 ACME challenges for Let's Encrypt.
#
# Usage:
#   ./create-iam-policy.sh <HOSTED_ZONE_ID>
#
# Example:
#   ./create-iam-policy.sh Z1D633PJN98FT9
#
# After running this script, attach the printed policy ARN to the IAM identity
# that the Caddy container authenticates as (EC2 instance role, ECS task role,
# or IAM user).  Then set AWS_HOSTED_ZONE_ID to the same zone ID in
# web/docker-compose.yml (uncomment the relevant line) so the plugin skips
# zone-discovery calls and the policy stays narrow.
#
# If you cannot set AWS_HOSTED_ZONE_ID (Variant B), add this statement to the
# policy so the plugin can discover the zone by domain name:
#
#   {
#     "Sid": "Route53ListZones",
#     "Effect": "Allow",
#     "Action": [
#       "route53:ListHostedZonesByName",
#       "route53:ListHostedZones"
#     ],
#     "Resource": "*"
#   }
#
# Note: list/discovery actions do not support resource-level restrictions;
# "Resource": "*" is required by AWS for those actions.

set -euo pipefail

POLICY_NAME="caddy-route53-dns-challenge"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
POLICY_FILE="${SCRIPT_DIR}/iam-caddy-route53.json"

# ── Validate arguments ────────────────────────────────────────────────────────
if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <HOSTED_ZONE_ID>" >&2
    exit 1
fi

HOSTED_ZONE_ID="$1"

# ── Resolve account ID and policy ARN ────────────────────────────────────────
ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"
POLICY_ARN="arn:aws:iam::${ACCOUNT_ID}:policy/${POLICY_NAME}"

# ── Build policy document with the real zone ID ───────────────────────────────
POLICY_DOCUMENT="$(sed "s/HOSTED_ZONE_ID/${HOSTED_ZONE_ID}/g" "${POLICY_FILE}")"

# ── Create or update policy ───────────────────────────────────────────────────
if aws iam get-policy --policy-arn "${POLICY_ARN}" &>/dev/null; then
    # IAM allows at most 5 versions per policy. Delete the oldest non-default
    # version if the limit is reached before creating the new one.
    OLDEST_NON_DEFAULT="$(aws iam list-policy-versions \
        --policy-arn "${POLICY_ARN}" \
        --query "sort_by(Versions, &CreateDate)[?IsDefaultVersion==\`false\`] | [0].VersionId" \
        --output text)"

    VERSION_COUNT="$(aws iam list-policy-versions \
        --policy-arn "${POLICY_ARN}" \
        --query "length(Versions)" \
        --output text)"

    if [[ "${VERSION_COUNT}" -ge 5 && "${OLDEST_NON_DEFAULT}" != "None" ]]; then
        aws iam delete-policy-version \
            --policy-arn "${POLICY_ARN}" \
            --version-id "${OLDEST_NON_DEFAULT}"
    fi

    aws iam create-policy-version \
        --policy-arn "${POLICY_ARN}" \
        --policy-document "${POLICY_DOCUMENT}" \
        --set-as-default \
        --query "PolicyVersion.VersionId" \
        --output text
    echo "Policy updated: ${POLICY_ARN}"
else
    aws iam create-policy \
        --policy-name "${POLICY_NAME}" \
        --description "Allows Caddy to complete Route53 DNS-01 ACME challenges for Let's Encrypt" \
        --policy-document "${POLICY_DOCUMENT}" \
        --query "Policy.Arn" \
        --output text
    echo "Policy created: ${POLICY_ARN}"
fi
