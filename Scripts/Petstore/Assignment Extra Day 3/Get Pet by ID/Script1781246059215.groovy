import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
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
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.util.KeywordUtil
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import static org.assertj.core.api.Assertions.assertThat


def response = WS.sendRequest(findTestObject('Petstore/Assignment Extra Day 3/Get-Pet', [('pet_id') : GlobalVariable.pet_id]))


WS.verifyResponseStatusCode(response, 200)

def json = new JsonSlurper().parseText(response.getResponseBodyContent())

WS.verifyElementPropertyValue(response, 'id', 23)
WS.verifyElementPropertyValue(response, 'category.name', "Kucing")
WS.verifyElementPropertyValue(response, 'name', "Ochi")
WS.verifyElementPropertyValue(response, 'status', "available")
WS.verifyElementPropertyValue(response, 'tags[0].name', "Persia")

assertThat(json.id as String).isNotNull()
assertThat(json.status).isNotNull()
assertThat(json.status).isNotEmpty()

println("========= LOG TEST CASE: GET PET BY ID SUCCESS =========")
println("Pet ID: " + json.id)
println("Pet Name: " + json.name)
println("Status: " + json.status)
println("=========================================================")
