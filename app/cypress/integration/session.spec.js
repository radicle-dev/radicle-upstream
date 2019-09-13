context('Session', () => {
  beforeEach(() => {
    cy.visit('http://localhost:8000')
  })

  it('allows to sign in', () => {
    cy.get('button').contains('Join the network').click()

    cy.get('input:first').should('have.attr', 'placeholder', 'Enter your name')
    cy.get('input:last').should('have.attr', 'placeholder', 'Enter an avatar URL')

    cy.get('input:first').type("Rudolfs Osins")
    cy.get('button').contains('Join the network').click()
    cy.get('header p').should('contain', 'Rudolfs Osins')
  })
})
