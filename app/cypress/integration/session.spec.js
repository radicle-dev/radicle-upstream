context('session', () => {
  beforeEach(() => {
    cy.visit('/')
  })

  Cypress.Commands.add('signIn', (username) => {
    cy.get('button').contains('Join the network').click()

    cy.get('input[placeholder="Enter your name"]').type(username)
    cy.get('button').contains('Join the network').click()
  })

  context('happy path', () => {
    it('signs in the user', () => {
      cy.signIn("Rudolfs")

      cy.get('header').contains('Rudolfs').should('exist')
      cy.contains('Welcome Rudolfs').should('exist')
    })

    it('forces user to sign in before registering a project', () => {
      cy.get('button').contains('Register project').click()

      cy.get('h2').contains('Join the network').should('exist')
    })

    it('does not lose the session over a browser reload', () => {
      cy.signIn("Ange")

      cy.get('header').contains('Ange').should('exist')

      cy.reload()
      cy.get('header').contains('Ange').should('exist')
    })

    it('signs out the user when localStorage is cleared', () => {
      cy.signIn("Merle")

      cy.get('header').contains('Merle').should('exist')

      cy.clearLocalStorage()
      cy.reload()
      cy.get('header').contains('Merle').should('not.exist')
    })
  })
})
