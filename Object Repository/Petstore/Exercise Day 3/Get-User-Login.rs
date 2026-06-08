<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get-User-Login</name>
   <tag></tag>
   <elementGuidId>3bbe7ab2-ca8e-4c8a-8963-0020bc4fbb5f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>659dcdb9-5d55-481d-b034-6a8caaf80011</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.1.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://petstore.swagger.io/v2/user/login?username=${param_username}&amp;password=${param_password}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>c4c3d13b-5612-48fb-8921-712ce7aaeba1</id>
      <name>Schema Validation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Schema/SchemeExercise.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.current_username</defaultValue>
      <description></description>
      <id>dbbf34c4-d5a8-48e1-bd76-4329538142fe</id>
      <masked>false</masked>
      <name>param_username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.current_password</defaultValue>
      <description></description>
      <id>53dca755-61f0-4f66-8730-6428826469b8</id>
      <masked>false</masked>
      <name>param_password</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'code', 200)
WS.verifyElementPropertyValue(response, 'type', &quot;unknown&quot;)

// Assert buat &quot;message&quot;
String responseText = response.getResponseBodyContent()


def json = new JsonSlurper().parseText(responseText)
String actualMessage = json.message


String expectedPattern = /logged in user session:\d+/

WS.verifyMatch(actualMessage, expectedPattern, true)





</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
