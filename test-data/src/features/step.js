// src/step.js

import { Header as BigHeader, Footer } from '../shared'

export const Step = () => (
    <>
        <BigHeader />
    </>
)

export const StepList = () => (
    <>
        <Step />
        <Step />
        <Footer />
    </>
)

export const StepList2 = () => (
    <BigHeader>
        <Step />
    </BigHeader>
)

export function StepList3() {
    return (
        <>
            <Step />
            <Step />
            <Footer />
        </>
    )
}

export default StepList