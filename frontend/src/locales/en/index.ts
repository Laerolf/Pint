export default {
  pages: {
    customers: {
      overview: {
        title: 'Customers',
        table: {
          headers: {
            lastName: 'Last name',
            firstName: 'First name',
            nickname: 'Nickname'
          }
        }
      }
    }
  },
  sections: {
    customers: {
      detail: {
        sections: {
          about: {
            title: 'About',
            fields: {
              firstName: 'First name',
              lastName: 'Last name',
              nickname: 'Nickname',
              dateOfBirth: 'Date of birth'
            }
          },
          contact: {
            title: 'Contact',
            fields: {
              emailAddress: 'Email address'
            }
          }
        }
      }
    },
    meta: {
      title: 'Meta',
      fields: {
        source: 'Source',
        sourceId: 'Source ID',
        createdAt: 'Created at',
        lastUpdatedAt: 'Last updated at'
      }
    }
  }
}
