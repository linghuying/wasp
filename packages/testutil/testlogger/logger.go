// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

package testlogger

import (
	"testing"

	"github.com/iotaledger/hive.go/logger"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

// NewLogger produces a logger adjusted for test cases.
func NewLogger(t *testing.T, timeLayout ...string) *logger.Logger {
	return NewNamedLogger(t.Name())
}

// NewNamedLogger produces a logger adjusted for test cases.
func NewNamedLogger(name string, timeLayout ...string) *logger.Logger {
	// log, err := zap.NewDevelopment()
	cfg := zap.NewDevelopmentConfig()
	if len(timeLayout) > 0 {
		cfg.EncoderConfig.EncodeTime = zapcore.TimeEncoderOfLayout(timeLayout[0])
	}
	log, err := cfg.Build()
	if err != nil {
		panic(err)
	}
	return log.Named(name).Sugar()
}

// WithLevel returns a logger with a level increased.
// Can be useful in tests to disable logging in some parts of the system.
func WithLevel(log *logger.Logger, level logger.Level, printStackTrace bool) *logger.Logger {
	if printStackTrace {
		return log.Desugar().WithOptions(zap.IncreaseLevel(level), zap.AddStacktrace(zapcore.PanicLevel)).Sugar()
	}
	return log.Desugar().WithOptions(zap.IncreaseLevel(level), zap.AddStacktrace(zapcore.FatalLevel)).Sugar()
}
