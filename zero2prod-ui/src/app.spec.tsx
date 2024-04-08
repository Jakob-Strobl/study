import { render } from '@solidjs/testing-library';
import App from './app'

describe("Test the render <App />", () => {
    test("smoke render test", () => {
        const component = render(() => (
            <App></App>
        ))

        expect(component.getByText("Hello world!")).toBeVisible()
        expect(component.baseElement.querySelector('button')?.innerHTML).toEqual("Clicks: 0")
        expect(component.baseElement.querySelector('p')?.innerHTML).toEqual('Visit <a href=\"https://start.solidjs.com\" target=\"_blank\">start.solidjs.com</a> to learn how to build SolidStart apps.')
    })

    test("Counter increments when button is clicked", () => {
        const component = render(() => (
            <App></App>
        ))

        const button = component.baseElement.querySelector('button') 
        expect(button?.innerHTML).toEqual("Clicks: 0")
        button?.click()
        expect(button?.innerHTML).toEqual("Clicks: 1")
    })
})