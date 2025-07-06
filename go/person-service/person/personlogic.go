package person

import (
	"gofr.dev/pkg/gofr"
)

func Search(ctx *gofr.Context, firstName string, lastName string) (any, error) {
	err := DemoData(ctx)
	if err != nil {
		return nil, err
	}

	return FindAll(ctx, firstName, lastName)
}
