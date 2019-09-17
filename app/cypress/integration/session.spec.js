context('session', () => {
  beforeEach(() => {
    cy.visit('http://localhost:8000')
  })

  it('signs in the user', () => {
    cy.get('button').contains('Join the network').click()

    cy.get('input[placeholder="Enter your name"]').type("Rudolfs")
    cy.get('button').contains('Join the network').click()

    cy.get('header').contains('Rudolfs').should('exist')
    cy.contains('Welcome Rudolfs').should('exist')
  })
})
