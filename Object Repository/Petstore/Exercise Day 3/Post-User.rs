<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post-User</name>
   <tag></tag>
   <elementGuidId>f060f2e8-7ce9-4b65-a11a-a9ba1216945f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: 205,\n  \&quot;username\&quot;: \&quot;yumandaevan\&quot;,\n  \&quot;firstName\&quot;: \&quot;devan\&quot;,\n  \&quot;lastName\&quot;: \&quot;nabana\&quot;,\n  \&quot;email\&quot;: \&quot;yumandaevn@gmail.com\&quot;,\n  \&quot;password\&quot;: \&quot;epanganteng\&quot;,\n  \&quot;phone\&quot;: \&quot;0882123238\&quot;,\n  \&quot;userStatus\&quot;: 0\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a10876e8-bba8-453a-8ce6-08f2b248b94a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a90e4c03-10b0-433f-bf4b-08c5c4c2fb29</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>11.1.3</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://petstore.swagger.io/v2/user</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>12062720-b904-4cb0-bbf8-95375f2de441</id>
      <name>Schema Validation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>Schema/SchemeExercise.json</data>
      <activate>true</activate>
   </validationSteps>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable


RequestObject requestObj = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject responseObj = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(responseObj, 200)
assertThat(responseObj.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(responseObj, 'code', 200)
WS.verifyElementPropertyValue(responseObj, 'type', &quot;unknown&quot;)
WS.verifyElementPropertyValue(responseObj, 'message', 205)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
