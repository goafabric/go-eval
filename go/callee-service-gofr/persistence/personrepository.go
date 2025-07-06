package persistence

import (
	"gofr.dev/pkg/gofr"
	"github.com/google/uuid"
)

func RoutePerson(app *gofr.App) {
    app.GET("/persons", func(ctx *gofr.Context) (interface{}, error) {
        err := demoData(ctx)
        if err != nil {
            return nil, err
        }
        return findAll(ctx)
	})
}

func demoData(ctx *gofr.Context) (error) {
    err := save(ctx, Person{ID: uuid.New().String(), OrganizationId: "1", FirstName: "Homer", LastName: "Simpson"})
    return err
}

func save(ctx *gofr.Context, person Person) (error) {
	_, err := ctx.SQL.ExecContext(ctx, "INSERT INTO example_0.person (id, organization_id, first_name, last_name) VALUES ($1, $2, $3, $4)",
	    person.ID, person.OrganizationId, person.FirstName, person.LastName)

    return err
}

func findAll(ctx *gofr.Context) (any, error) {
    var persons []Person
    
	rows, err := ctx.SQL.QueryContext(ctx, "SELECT id, first_name, last_name FROM example_0.person")
    if err != nil {
        return nil, err
    }

    for rows.Next() {
        var person Person
        if err := rows.Scan(&person.ID, &person.FirstName, &person.LastName); err != nil {
            return nil, err
        }

        persons = append(persons, person)
    }
    return persons, nil

}