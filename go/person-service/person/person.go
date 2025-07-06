package person

type Person struct {
	ID             string `json:"id"`
	OrganizationId string `json:"organization_id"`

	FirstName string `json:"first_name"`
	LastName  string `json:"last_name"`
}
