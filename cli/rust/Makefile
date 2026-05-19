# Copyright (C) 2026 Red Hat, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# SPDX-License-Identifier: Apache-2.0

.PHONY: clean gen all

all: gen

gen: clean
	mkdir -p src
	yq -o json '{"$$schema": "http://json-schema.org/draft-07/schema#", "definitions": .components.schemas}' \
		../openapi.yaml | sed 's|#/components/schemas/|#/definitions/|g' > /tmp/gen-schema.yaml
	cargo typify /tmp/gen-schema.yaml --output src/lib.rs
	rm /tmp/gen-schema.yaml

clean:
	rm -f src/lib.rs
