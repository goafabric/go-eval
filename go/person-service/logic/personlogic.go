package logic

import (
	"gofr.dev/pkg/gofr"

	"person-service/persistence"
)

func Search(ctx *gofr.Context, firstName string, lastName string) (any, error) {
	err := persistence.DemoData(ctx)
	if err != nil {
		return nil, err
	}

	return persistence.FindAll(ctx, firstName, lastName)
}
