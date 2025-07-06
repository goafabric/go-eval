package persistence

import (
	"person-service/controller/dto"

	"github.com/google/uuid"
	"gofr.dev/pkg/gofr"
)

func DemoData(ctx *gofr.Context) error {
	err := createPersonTable(ctx)
	if err != nil {
		return err
	}

	rows, _ := ctx.SQL.QueryContext(ctx, "SELECT * FROM example_0.person")
	if rows.Next() {
		return nil
	}

	err2 := save(ctx, dto.Person{ID: uuid.New().String(), OrganizationId: "1", FirstName: "Homer", LastName: "Simpson"})
	save(ctx, dto.Person{ID: uuid.New().String(), OrganizationId: "1", FirstName: "Bart", LastName: "Simpson"})
	return err2
}

func save(ctx *gofr.Context, person dto.Person) error {
	_, err := ctx.SQL.ExecContext(ctx, "INSERT INTO example_0.person (id, organization_id, first_name, last_name) VALUES ($1, $2, $3, $4)",
		person.ID, person.OrganizationId, person.FirstName, person.LastName)

	return err
}

func createPersonTable(ctx *gofr.Context) error {
	query := `
	CREATE SCHEMA IF NOT EXISTS example_0;

	CREATE TABLE IF NOT EXISTS example_0.person (
		id VARCHAR(36) NOT NULL PRIMARY KEY,
		organization_id VARCHAR(36) NOT NULL,
		first_name VARCHAR(255),
		last_name VARCHAR(255),
		version BIGINT DEFAULT 0
	);
	`

	_, err := ctx.SQL.ExecContext(ctx, query)
	return err
}
