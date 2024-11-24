// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package genclient

import (
	"fmt"
	"os"

	"github.com/ghodss/yaml"
	"google.golang.org/genproto/googleapis/api/serviceconfig"
	"google.golang.org/protobuf/encoding/protojson"
)

func ReadServiceConfig(serviceConfigPath string) (*serviceconfig.Service, error) {
	y, err := os.ReadFile(serviceConfigPath)
	if err != nil {
		return nil, fmt.Errorf("error reading service config: %v", err)
	}

	j, err := yaml.YAMLToJSON(y)
	if err != nil {
		return nil, fmt.Errorf("error converting YAML to JSON: %v", err)
	}

	cfg := &serviceconfig.Service{}
	if err := (protojson.UnmarshalOptions{DiscardUnknown: true}).Unmarshal(j, cfg); err != nil {
		return nil, fmt.Errorf("error unmarshalling service config: %v", err)
	}

	// An API Service Config will always have a `name` so if it is not populated,
	// it's an invalid config.
	if cfg.GetName() == "" {
		return nil, fmt.Errorf("invalid API service config file %q", serviceConfigPath)
	}
	return cfg, nil
}
