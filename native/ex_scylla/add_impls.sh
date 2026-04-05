#!/bin/bash
add_impl() {
    file=$1
    struct=$2
    if ! grep -q "impl rustler::Resource for $struct" "$file"; then
        echo "impl rustler::Resource for $struct {}" >> "$file"
    fi
}

add_impl src/batch/types.rs BatchResource
add_impl src/execution/execution_profile.rs ExecutionProfileResource
add_impl src/execution/execution_profile_builder.rs ExecutionProfileBuilderResource
add_impl src/execution/execution_profile_handle.rs ExecutionProfileHandleResource
add_impl src/execution/load_balancing.rs DefaultPolicyBuilderResource
add_impl src/execution/load_balancing.rs LatencyAwarenessPolicyBuilderResource
add_impl src/execution/load_balancing.rs LoadBalancingPolicyResource
add_impl src/prepared_statement/types.rs PreparedStatementResource
add_impl src/query/types.rs QueryResource
add_impl src/session/types.rs SessionResource
add_impl src/session/types.rs ScyllaRawRowsResource
add_impl src/session_builder/types.rs SessionBuilderResource
