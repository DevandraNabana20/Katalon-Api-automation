import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.azure.core.util.Header
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import com.networknt.schema.utils.JsonType
import com.sun.tools.javac.util.Assert

import groovy.json.JsonSlurper
import internal.GlobalVariable
import io.restassured.internal.common.assertion.Assertion
import io.restassured.response.ResponseBody
import net.bytebuddy.agent.builder.AgentBuilder.CircularityLock.Global

import org.openqa.selenium.Keys as Keys

def JsonType = GlobalVariable.ContentType

//1. definisi method

def userRequest =  findTestObject('Petstore/Post-Order-Store')

def sendRequest = WS.sendRequest(userRequest)

WS.verifyResponseStatusCode(sendRequest, 200)

//2. definisi request body
def responseBody = sendRequest.getResponseBodyContent()

def jsonBody = new JsonSlurper().parseText(responseBody)

def header = 
{
	"Content-Type" JsonType
}


//3. definisi Assertion response

assert jsonBody.id == 4
assert jsonBody.petId == 5
assert jsonBody.quantity == 70

//4. definisi Assertion request

WS.verifyElementPropertyValue(sendRequest, 'status', 'placed')
WS.verifyElementPropertyValue(sendRequest, 'complete', true)

