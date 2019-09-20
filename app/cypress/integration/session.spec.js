context('session', () => {
  beforeEach(() => {
    cy.visit('/')
  })

  let signIn = (username) => {
    cy.get('button').contains('Join the network').click()

    cy.get('input[placeholder="Enter your name"]').type(username)
    cy.get('button').contains('Join the network').click()
  }

  context('happy path', () => {
    it('signs in the user', () => {
      signIn("Rudolfs")

      cy.get('header').contains('Rudolfs').should('exist')
      cy.contains('Welcome Rudolfs').should('exist')
    })

    it('forces user to sign in before registering a project', () => {
      cy.get('button').contains('Register project').click()

      cy.get('h2').contains('Join the network').should('exist')
    })

    it('does not lose the session over a browser reload', () => {
      signIn("Ange")

      cy.get('header').contains('Ange').should('exist')

      cy.reload()
      cy.get('header').contains('Ange').should('exist')
    })

    it('signs out the user when localStorage is cleared', () => {
      signIn("Ange")

      cy.get('header').contains('Ange').should('exist')

      cy.clearLocalStorage()
      cy.reload()
      cy.get('header').contains('Ange').should('not.exist')
    })
  })
})
