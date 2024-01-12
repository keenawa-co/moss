package validator

import (
	"fmt"
	"strings"

	"github.com/go-playground/validator/v10"
)

type validate interface {
	Struct(s interface{}) error
}

type ValidatorService struct {
	v validate
}

func NewValidatorService(v validate) *ValidatorService {
	return &ValidatorService{
		v: v,
	}
}

func (vs *ValidatorService) ValidateStruct(s interface{}) error {
	if err := vs.v.Struct(s); err != nil {
		if validationErrors, ok := err.(validator.ValidationErrors); ok {
			var errMessages []string
			for _, valErr := range validationErrors {
				errMessages = append(errMessages, fmt.Sprintf("field '%s' with condition: '%s'", valErr.Field(), valErr.ActualTag()))
			}

			return fmt.Errorf(strings.Join(errMessages, ", "))
		}
	}

	return nil
}
